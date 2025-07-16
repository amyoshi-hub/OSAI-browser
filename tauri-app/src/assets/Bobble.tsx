import React, { useEffect, useState } from 'react';
import './Bubble.css';

const Bubble = () => {
  const [bubbles, setBubbles] = useState<number[]>([]);

  useEffect(() => {
    const interval = setInterval(() => {
      setBubbles((prev) => [...prev, Math.random()]);
    }, 800);

    return () => clearInterval(interval);
  }, []);

  return (
    <div className="bubble-container">
      {bubbles.map((id, i) => (
        <span
          key={i}
          className="bubble"
          style={{ left: `${Math.random() * 100}%`, animationDelay: `${Math.random() * 3}s` }}
        ></span>
      ))}
    </div>
  );
};

export default Bubble;

