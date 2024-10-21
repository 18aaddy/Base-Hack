import React, { useState, useEffect } from 'react';
import { BrowserProvider, isAddress } from 'ethers';
import { BrowserRouter as Router, Route, Routes } from 'react-router-dom';
import { Search } from 'lucide-react';
import notification from './assets/notification.png';
import gas from './assets/gas.png';
import Sidebar from './components/SideBar';
import Dashboard from './components/Dashboard';
import Tokens from './components/Tokens';
import Transactions from './components/Transactions';
import NFTShowcase from './components/NFTs';

const App = () => {
  const [userAddress, setUserAddress] = useState(null);
  const [inputAddress, setInputAddress] = useState('');
  const [erc20Data, setErc20Data] = useState(null);
  const [erc721Data, setErc721Data] = useState(null);
  const [transactionData, setTransactionData] = useState(null);
  const [loading, setLoading] = useState(false);

  const handleConnectWallet = async () => {
    if (window.ethereum) {
      try {
        await window.ethereum.request({ method: 'eth_requestAccounts' });
        const provider = new BrowserProvider(window.ethereum); // Updated
        const signer = await provider.getSigner();
        const address = await signer.getAddress();
        console.log('Connected address:', address);
        setUserAddress(address);
      } catch (error) {
        console.error('Error connecting to wallet:', error);
      }
    } else {
      console.error('MetaMask is not installed');
    }
  };

  const handleSearch = () => {
    if (isAddress(inputAddress)) { // Updated
      setUserAddress(inputAddress);
    } else {
      alert('Please enter a valid Ethereum address');
    }
  };

  useEffect(() => {
    if (userAddress) {
      fetchData();
    }
  }, [userAddress]);

  const fetchData = async () => {
    setLoading(true);
    try {
      const chain = 'BASE';
      const request = {
        user_address: userAddress,
        chain: chain,
      };
      const LOCALHOST = '127.0.0.1';
      const port = 8080;

      // Fetch ERC20 data
      const erc20Response = await fetch(`http://${LOCALHOST}:${port}/erc20`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(request),
      });
      const erc20Result = await erc20Response.json();
      setErc20Data(erc20Result);

      // Fetch ERC721 data
      const erc721Response = await fetch(`http://${LOCALHOST}:${port}/erc721`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(request),
      });
      const erc721Result = await erc721Response.json();
      setErc721Data(erc721Result);

      // Fetch Transaction data
    const transactionResponse = await fetch(
      `http://${LOCALHOST}:${port}/transaction-history`,
      {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(request),
      }
    );

    if (!transactionResponse.ok) {
      const errData = await transactionResponse.json();
      console.error('Transaction fetch error:', errData);
      throw new Error(errData.message || 'Unknown error occurred');
    }

    const transactionResult = await transactionResponse.json();
    console.log('Transaction Data:', transactionResult); // Add this line
    setTransactionData(transactionResult);

  } catch (error) {
    console.error('Error fetching data:', error);
  }
  setLoading(false);
};

  return (
    <Router>
      <div className="flex bg-black text-white min-h-screen">
        <div className="fixed left-0 top-0 h-screen">
          <Sidebar />
        </div>
        <div className="flex-1 ml-[25vw] mt-[8vh] p-8">
          <h1
            className="text-[36px] font-inter ml-4 mt-[-8px] font-semibold"
            style={{ color: 'rgba(255, 255, 255, 0.70)' }}
          >
            Welcome Back, Anon
          </h1>
          <header className="flex mt-4 justify-end items-center mb-8">
            <div className="flex items-center space-x-4">
              <div className="relative">
                <input
                  type="text"
                  placeholder="Search an address..."
                  className="bg-[#131313] rounded-md border-[1px] border-solid border-[rgba(52,52,52,0.58)] py-2 px-4 pr-10"
                  value={inputAddress}
                  onChange={(e) => setInputAddress(e.target.value)}
                  onKeyDown={(e) => {
                    if (e.key === 'Enter') {
                      handleSearch();
                    }
                  }}
                />
                <button
                  onClick={handleSearch}
                  className="absolute right-3 top-2.5 text-gray-400"
                >
                  <Search size={20} />
                </button>
              </div>
              <div className="bg-[#131313] rounded-md border-[1px] border-solid border-[rgba(52,52,52,0.58)] p-2 flex">
                <img src={gas} alt="gas Icon" className="h-6 w-6 mr-1" />
                <span>| 12</span>
              </div>
              <div className="bg-[#131313] rounded-md border-[1px] border-solid border-[rgba(52,52,52,0.58)] p-2 relative">
                <img src={notification} alt="Wallet Icon" className="h-6 w-6" />
              </div>
              <button
                onClick={handleConnectWallet}
                className="bg-[#DA0046] text-white font-bold rounded-md w-32 px-4 py-2"
              >
                My Wallet
              </button>
            </div>
          </header>
          <Routes>
            <Route
              path="/"
              element={
                <Dashboard
                  erc20Data={erc20Data}
                  erc721Data={erc721Data}
                  transactionData={transactionData}
                  loading={loading}
                />
              }
            />
            <Route
              path="/tokens"
              element={<Tokens erc20Data={erc20Data} loading={loading} />}
            />
             <Route
              path="/transactions"
              element={
                <Transactions/>
              }
            />
              <Route
              path="/Nfts"
              element={<NFTShowcase />}
              />
          </Routes>
        </div>
      </div>
    </Router>
  );
};

export default App;
