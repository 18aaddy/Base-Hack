import React from 'react';
import { Bitcoin } from 'lucide-react';
import FloatingOverlay from './Tkn';

const Tokens = ({ erc20Data, loading }) => {
  if (loading) {
    return (
      <div className="flex justify-center items-center h-full">
        <p>Loading...</p>
      </div>
    );
  }

  if (!erc20Data) {
    return <p>No data available.</p>;
  }

  const erc20Details = erc20Data.user_details.details;
  const erc20PortfolioValue = erc20Data.user_details.portfolio_value;

  const tokens = erc20Details.map((tokenDetail) => {
    const tokenName = tokenDetail.token_name;
    const tokenSymbol = tokenDetail.token_symbol;
    const tokenBalance =
      parseInt(tokenDetail.token_balance, 16) /
      Math.pow(10, tokenDetail.token_decimals);
    const tokenPrice = tokenDetail.token_price || 0;
    const tokenValue = tokenBalance * tokenPrice;
    const percentage = ((tokenValue / erc20PortfolioValue) * 100).toFixed(2) + '%';

    return {
      name: tokenSymbol,
      percentage: percentage,
      priceChange: '+$0 (0%)', // Placeholder
      price: `$${tokenPrice.toFixed(2)}`,
      balance: `${tokenBalance.toFixed(4)} ${tokenSymbol}`,
    };
  });

  return (
    <div>
      <main className="bg-[#131313] p-2 rounded-3xl rounded-tl-none pt-6 mt-[60px]">
        <div className="bg-[#1E1E1E] p-4 pl-10 ml-[10px] mr-[15px] pr-0 mb-8 rounded-lg">
          <div className="grid grid-cols-2 gap-4">
            <div className="bg-[#232323] ml-[-25px] rounded-lg w-[460px] p-4 py-2">
              <h3 className="text-white font-inter font-semibold mb-2">
                Net Worth
              </h3>
              <div className="flex flex-col">
                <span className="text-4xl font-inter font-bold mb-2">
                  ${erc20PortfolioValue.toFixed(2)}
                </span>
                <div className="text-green-500 font-inter font-semibold text-sm">
                  +0% ($0)
                </div>
              </div>
            </div>
            {/* Add other cards as needed */}
          </div>
        </div>

        <div className="bg-transparent p-4 ml-[-6px] rounded-lg mt-[-38px]">
          <div className="bg-[#1E1E1E] rounded-lg p-4">
            {tokens.map((token, index) => (
              <div
                key={index}
                className="flex items-center justify-between py-4 px-3 bg-[#232323] rounded-lg mb-2"
              >
                <div className="flex items-center">
                  <div className="w-8 h-8 mr-3 bg-gray-600 rounded-full flex items-center justify-center">
                    <Bitcoin size={20} />
                  </div>
                  <div>
                    <div className="font-semibold">{token.name}</div>
                    <div className="text-sm text-gray-400">Token</div>
                  </div>
                </div>
                <div className="text-right">
                  <div>{token.percentage}</div>
                  <div className="text-green-500 text-sm">
                    {token.priceChange}
                  </div>
                </div>
                <div className="text-right">
                  <div>{token.price}</div>
                  <div className="text-sm text-gray-400">{token.balance}</div>
                </div>
              </div>
            ))}
          </div>
        </div>
      </main>
      <FloatingOverlay />
    </div>
  );
};

export default Tokens;
