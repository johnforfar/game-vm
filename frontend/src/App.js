// frontend/src/App.js
import React, { useRef, useEffect, useState } from 'react';
import './App.css';

const PLAYER_SIZE = 30;
const FIREBALL_SIZE = 10;
const GRAVITY = 0.5;
const JUMP_VELOCITY = -10;

// Initially, the game fee is set to 100 kin for a normal join. 
// For our AI Agent we'll use 1 kin.
const GAME_FEE = 100;
const AI_FEE = 1;

function drawHexagon(ctx, x, y, radius, color) {
  const sides = 6;
  const angleOffset = Math.PI / 2; // so the top vertex points straight up
  ctx.beginPath();
  for (let i = 0; i < sides; i++) {
    const angle = (2 * Math.PI * i) / sides - angleOffset;
    const px = x + radius * Math.cos(angle);
    const py = y + radius * Math.sin(angle);
    if (i === 0) {
      ctx.moveTo(px, py);
    } else {
      ctx.lineTo(px, py);
    }
  }
  ctx.closePath();
  ctx.fillStyle = color;
  ctx.fill();
}

function App() {
  const canvasRef = useRef(null);
  const [gameStarted, setGameStarted] = useState(false);
  const [joinedPool, setJoinedPool] = useState(false);
  const [score, setScore] = useState(0);
  
  // game objects
  const player = useRef({
    x: 50,
    y: 200,
    vy: 0,
    onGround: true,
  });
  const fireballs = useRef([]);
  const enemies = useRef([
    { x: 400, y: 370, speed: 1 },
    { x: 600, y: 370, speed: 1 },
  ]);
  
  // We'll use a ref to keep track of score in the game loop.
  const scoreRef = useRef(0);
  let spawnTimer = 0; // Used to time enemy spawns
  
  // Dummy function to simulate joining the game pool and paying a fee.
  const joinGamePool = async (fee) => {
    alert(`Joining game pool and paying a fee of ${fee} kin.`);
    setJoinedPool(true);
  };
  
  // This function is only used for the AI Agent.
  const deployAIAgent = async () => {
    await joinGamePool(AI_FEE);
  };
  
  useEffect(() => {
    if (!gameStarted) return;
    
    const canvas = canvasRef.current;
    const ctx = canvas.getContext('2d');
    let animationFrameId;
    let bgOffset = 0; // for background scrolling
    
    const handleKeyDown = (e) => {
      if (e.code === 'ArrowUp' && player.current.onGround) {
        player.current.vy = JUMP_VELOCITY;
        player.current.onGround = false;
      }
      // Press space to throw a fireball
      if (e.code === 'Space') {
        fireballs.current.push({
          x: player.current.x,
          y: player.current.y,
          vx: 5,
        });
      }
    };
    
    window.addEventListener('keydown', handleKeyDown);
    
    const update = () => {
      // Update background scroll
      bgOffset = (bgOffset + 0.5) % 200;
      
      // Update player physics
      player.current.vy += GRAVITY;
      player.current.y += player.current.vy;
      if (player.current.y > canvas.height - PLAYER_SIZE) {
        player.current.y = canvas.height - PLAYER_SIZE;
        player.current.vy = 0;
        player.current.onGround = true;
      }
      
      // Update fireballs
      fireballs.current = fireballs.current.filter(fb => fb.x < canvas.width);
      fireballs.current.forEach(fb => {
        fb.x += fb.vx;
      });
      
      // Check collisions between fireballs and enemies
      for (let i = fireballs.current.length - 1; i >= 0; i--) {
        const fb = fireballs.current[i];
        for (let j = enemies.current.length - 1; j >= 0; j--) {
          const enemy = enemies.current[j];
          const dx = fb.x - enemy.x;
          const dy = fb.y - enemy.y;
          const dist = Math.sqrt(dx * dx + dy * dy);
          if (dist < (PLAYER_SIZE + FIREBALL_SIZE)) {
            // Collision: remove enemy and fireball, then add score
            fireballs.current.splice(i, 1);
            enemies.current.splice(j, 1);
            scoreRef.current += 10;
            setScore(scoreRef.current);
            break;
          }
        }
      }
      
      // Increase difficulty based on score
      const currentDifficulty = 1 + scoreRef.current / 100;
      
      // Update enemy positions and check for collisions with player
      enemies.current.forEach((enemy) => {
        enemy.x -= enemy.speed * currentDifficulty;
        const dx = enemy.x - player.current.x;
        const dy = enemy.y - player.current.y;
        const dist = Math.sqrt(dx * dx + dy * dy);
        if (dist < PLAYER_SIZE) {
          setGameStarted(false);
          alert(`Game Over! Final Score: ${scoreRef.current}`);
        }
      });
      
      // Remove enemies that have moved off screen
      enemies.current = enemies.current.filter(enemy => enemy.x > -50);
      
      // Spawn new enemies every ~3 seconds (assuming ~60 FPS)
      spawnTimer++;
      if (spawnTimer > 180) {
        enemies.current.push({
          x: canvas.width + Math.random() * 100,
          y: 370,
          speed: 1,
        });
        spawnTimer = 0;
      }
    };
    
    const drawBackground = () => {
      // Draw sky
      ctx.fillStyle = '#87CEEB';
      ctx.fillRect(0, 0, canvas.width, canvas.height);
      
      // Draw moving hills
      ctx.fillStyle = '#228B22';
      for (let i = -1; i < canvas.width / 200 + 2; i++) {
        const hillX = i * 200 - bgOffset;
        ctx.beginPath();
        ctx.moveTo(hillX, canvas.height - 20);
        ctx.lineTo(hillX + 100, canvas.height - 150);
        ctx.lineTo(hillX + 200, canvas.height - 20);
        ctx.closePath();
        ctx.fill();
      }
    };
    
    const draw = () => {
      drawBackground();
      // Draw ground
      ctx.fillStyle = '#444';
      ctx.fillRect(0, canvas.height - 20, canvas.width, 20);
      
      // Draw player
      drawHexagon(ctx, player.current.x, player.current.y, PLAYER_SIZE, '#00AACC');
      
      // Draw fireballs
      fireballs.current.forEach(fb => {
        ctx.beginPath();
        ctx.arc(fb.x, fb.y, FIREBALL_SIZE, 0, 2 * Math.PI);
        ctx.fillStyle = '#FF4500';
        ctx.fill();
      });
      
      // Draw enemies
      enemies.current.forEach(enemy => {
        drawHexagon(ctx, enemy.x, enemy.y, PLAYER_SIZE, '#AA0000');
      });
    };
    
    const gameLoop = () => {
      update();
      draw();
      animationFrameId = requestAnimationFrame(gameLoop);
    };
    
    gameLoop();
    
    return () => {
      window.removeEventListener('keydown', handleKeyDown);
      cancelAnimationFrame(animationFrameId);
    };
  }, [gameStarted]);
  
  return (
    <div className="App">
      <h1>Hexagon Challenge</h1>
      {!joinedPool ? (
        <div>
          <button onClick={() => joinGamePool(GAME_FEE)}>Join Game Pool (100 kin)</button>
          <button onClick={deployAIAgent}>Deploy AI Agent (1 kin)</button>
        </div>
      ) : (
        <p>Waiting for players...</p>
      )}
      {joinedPool && (
        <button onClick={() => setGameStarted(true)}>
          Start Game
        </button>
      )}
      <canvas ref={canvasRef} width={800} height={400} className="game-canvas" />
      <p>Score: {score}</p>
      <p>Use ↑ arrow to jump and Space to throw fireballs.</p>
    </div>
  );
}

export default App;