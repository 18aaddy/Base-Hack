import React from 'react';
import { NavLink } from 'react-router-dom';
import awardIcon from '../assets/award.png';
import dashboard from '../assets/dashboard.png';
import categoryIcon from '../assets/nft.png';
import transaction from '../assets/transactions.png';
import logo from '../assets/logo.png';

const Sidebar = () => {
  const sidebarOptions = [
    { name: 'Dashboard', iconSrc: dashboard, path: '/' },
    { name: 'Tokens', iconSrc: awardIcon, path: '/tokens' },
    { name: 'NFTs', iconSrc: categoryIcon, path: '/Nfts' },
    { name: 'Transactions', iconSrc: transaction, path: '/transactions' },
  ];

  return (
    <div className="fixed top-0 left-0 h-[93vh] w-[22vw] m-[45px] bg-[rgba(31,31,31,0.60)] rounded-[25px] text-white flex flex-col justify-between p-6">
      <div>
        <div className="m-[32px] mt-[35px] flex justify-center items-center">
          <img src={logo} alt="Athena Logo" className="h-10 mr-2" />
          <h1 className="text-[#DA0046] font-syncopate text-[35.909px] font-bold mt-3 leading-3 tracking-title">NEOM</h1>
        </div>
        <hr className="border-t-[0.5px] mt-12 mb-[28px] border-white w-full" />
        <nav>
          {sidebarOptions.map((option) => (
            <NavLink
              key={option.name}
              to={option.path}
              className={({ isActive }) => `
                flex items-center mb-[28px] cursor-pointer group relative
                ${isActive ? 'text-white' : 'text-[rgba(255,255,255,0.40)]'}
              `}
            >
              {({ isActive }) => (
                <>
                  {isActive && (
                    <div className="absolute left-[-24px] top-0 bottom-0 w-1 bg-[#DA0046] rounded-r-full"></div>
                  )}
                  <div className="flex items-center ml-4 font-inter font-semibold text-[18px] transition-all duration-200 ease-in-out hover:scale-105 hover:text-white">
                    <img src={option.iconSrc} alt={`${option.name} icon`} className="mr-3 h-5 w-5" />
                    <span>{option.name}</span>
                  </div>
                </>
              )}
            </NavLink>
          ))}
        </nav>
      </div>
    </div>
  );
};

export default Sidebar;