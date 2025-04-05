// Configuration
const numDots = 4000; // Number of dots
const svg = document.querySelector('svg');
const dotsGroup = document.getElementById('dots');
const wavesGroup = document.getElementById('waves');
const centerX = window.innerWidth / 2;
const centerY = window.innerHeight / 2;
const blackHoleRadius = 30; // Source-Dark size
const layers = 12; // Number of layers
const maxDotSize = 3; // Size of the largest dots
const minDotSize = 0.1; // Size of the smallest dots
const maxDistance = Math.max(centerX, centerY) * 0.95; // Maximum distance from the center
let dotsEntered = 0; // Counter for dots that have entered the Source-Dark
const escapeThreshold = 400; // Number of dots before a special dot (ISBE) escapes

// Impulse wave settings
const waveInterval = 4000; // Time between waves in milliseconds (every 3 seconds)
const waveDuration = 3000; // How long a wave takes to expand fully (1.5 seconds)
const maxWaveRadius = maxDistance * 1.5; // Maximum radius of the wave

// Create a dot with original continuous movement logic
function createDot(randomStart = true) {
  let startX, startY;
  if (randomStart) {
    const angle = Math.random() * 2 * Math.PI;
    const distance = Math.random() * maxDistance;
    startX = centerX + Math.cos(angle) * distance;
    startY = centerY + Math.sin(angle) * distance;
  } else {
    const angle = Math.random() * 2 * Math.PI;
    startX = centerX + Math.cos(angle) * maxDistance;
    startY = centerY + Math.sin(angle) * maxDistance;
  }
  
  const dot = document.createElementNS('http://www.w3.org/2000/svg', 'circle');
  dot.setAttribute('cx', startX);
  dot.setAttribute('cy', startY);
  dot.setAttribute('fill', 'black');
  dotsGroup.appendChild(dot);

  animateDot(dot, startX, startY);
}

// Original continuous motion animation function
function animateDot(dot, startX, startY) {
  const dx = centerX - startX;
  const dy = centerY - startY;
  const distance = Math.hypot(dx, dy);
  const directionX = dx / distance;
  const directionY = dy / distance;
  const baseSpeed = 0.1; // Base speed for smooth movement
  const speedVariance = 24; // Speed variance
  const speed = baseSpeed + Math.random() * speedVariance; // Randomize speed

  const moveDistance = () => {
    const moveX = directionX * speed * 0.005; // Assuming ~60fps
    const moveY = directionY * speed * 0.005;
    const currentX = parseFloat(dot.getAttribute('cx')) + moveX;
    const currentY = parseFloat(dot.getAttribute('cy')) + moveY;
    dot.setAttribute('cx', currentX);
    dot.setAttribute('cy', currentY);

    // Set size based on distance from the center
    const currentDistance = Math.hypot(centerX - currentX, centerY - currentY);
    const layer = Math.floor(currentDistance / maxDistance * layers);
    const newSize = minDotSize + (maxDotSize - minDotSize) * ((layers - layer) / layers);
    dot.setAttribute('r', newSize);

    // Check if the dot has entered the black hole
    if (currentDistance < blackHoleRadius - newSize) {
      dot.remove(); // Remove the dot when it reaches the center
      dotsEntered++;
      if (dotsEntered % escapeThreshold === 0) {
        createEscapeDot();
      } else {
        createDot(false); // New dots start from the outer-most edge
      }
    } else {
      requestAnimationFrame(moveDistance);
    }
  };

  moveDistance();
}

// Create an escape dot (unchanged from original)
function createEscapeDot() {
  const escapeDot = document.createElementNS('http://www.w3.org/2000/svg', 'circle');
  escapeDot.setAttribute('cx', centerX);
  escapeDot.setAttribute('cy', centerY);
  escapeDot.setAttribute('r', maxDotSize);
  escapeDot.setAttribute('fill', 'black');
  dotsGroup.appendChild(escapeDot);

  gsap.timeline()
    .to(escapeDot, { cy: centerY - 50, duration: 1 })
    .to(escapeDot, { cx: centerX + maxDistance, cy: centerY + maxDistance, r: 0, duration: 3, ease: 'power2.in' })
    .call(() => escapeDot.remove());
}

// Create an impulse wave (visual effect only)
function createImpulseWave() {
  const wave = document.createElementNS('http://www.w3.org/2000/svg', 'circle');
  wave.setAttribute('cx', centerX);
  wave.setAttribute('cy', centerY);
  wave.setAttribute('r', blackHoleRadius);
  wave.classList.add('impulse-wave');
  wavesGroup.appendChild(wave);

  // Animate the wave expansion
  gsap.to(wave, {
    r: maxWaveRadius,
    duration: waveDuration / 1000,
    ease: "power1.out",
    opacity: 0,
    attr: { 'stroke-width': 0.2 },
    onComplete: function() {
      wave.remove();
    }
  });
}

// Start emitting impulse waves periodically
function startImpulseWaves() {
  // Create the initial wave
  createImpulseWave();
  
  // Set up interval for additional waves
  setInterval(() => {
    createImpulseWave();
  }, waveInterval);
}

// Initial scatter with uniform distribution
for (let i = 0; i < numDots; i++) {
  createDot();
}

// Start impulse waves after a short delay
setTimeout(startImpulseWaves, 1000);

/* 
// COMMENTED OUT: Alternative Teleportation/Jump Movement Logic
// This is kept for reference to study the math behind it

// Function to make a dot jump when hit by a wave
function jumpAnimateDot(dot) {
  // Get current position
  const startX = parseFloat(dot.getAttribute('cx'));
  const startY = parseFloat(dot.getAttribute('cy'));
  
  // Calculate direction toward center
  const dx = centerX - startX;
  const dy = centerY - startY;
  const distance = Math.hypot(dx, dy);
  const directionX = dx / distance;
  const directionY = dy / distance;
  
  // Calculate jump distance - random between 30 and 150 pixels
  const jumpDistance = 30 + Math.random() * 120;
  
  // Make sure we don't jump past the center
  const actualJumpDistance = Math.min(jumpDistance, distance * 0.95);
  
  // Calculate new position
  const newX = startX + (directionX * actualJumpDistance);
  const newY = startY + (directionY * actualJumpDistance);
  
  // Immediately set to new position (discrete jump)
  dot.setAttribute('cx', newX);
  dot.setAttribute('cy', newY);
  
  // Update dot size based on new position
  const newDistance = Math.hypot(centerX - newX, centerY - newY);
  const layer = Math.floor(newDistance / maxDistance * layers);
  const newSize = minDotSize + (maxDotSize - minDotSize) * ((layers - layer) / layers);
  dot.setAttribute('r', newSize);
}

// Function to activate dots within the wave's range
function activateDotsInRange(innerRadius, outerRadius) {
  // Make sure the outer is always larger than inner
  if (innerRadius > outerRadius) {
    [innerRadius, outerRadius] = [outerRadius, innerRadius];
  }
  
  // Safety check - ensure we have a reasonable range
  const rangeWidth = outerRadius - innerRadius;
  if (rangeWidth < 0.5) return; // Too small to bother with
  
  // Find all dots within the wave's range
  const dots = dotsGroup.querySelectorAll('circle');
  dots.forEach(dot => {
    const dotX = parseFloat(dot.getAttribute('cx'));
    const dotY = parseFloat(dot.getAttribute('cy'));
    const distanceFromCenter = Math.hypot(centerX - dotX, centerY - dotY);
    
    // Check if dot is within the current wave range
    if (distanceFromCenter >= innerRadius && distanceFromCenter <= outerRadius) {
      // If dot is in the wave's path, make it jump
      jumpAnimateDot(dot);
    }
  });
}
*/
