import React from 'react';
import dash from '../assets/dash.jpeg'

const FloatingOverlay = () => {
  return (
    <div className="absolute w-80 top-[206px] left-[536px] transform -translate-x-1/2 -translate-y-1/2 z-50 select-none pointer-events-none">
      <div className="relative">
        <img 
          src={dash} 
          alt="Dashboard Overlay" 
          className="opacity-100"
        />
        <div className="absolute top-1/2 left-[110px] font-inter  transform -translate-x-1/2 -translate-y-1/2" >
          <h1 className="text-2xl font-medium tracking-wider" style={{ color: 'rgba(255, 255, 255, 0.80)' }}>
            Transactions
          </h1>
        </div>
      </div>
    </div>
  );
};

export default FloatingOverlay;