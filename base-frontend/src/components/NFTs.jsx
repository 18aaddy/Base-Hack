import React from 'react';
import FloatingOverlay from './N';
import { ChevronDown } from 'lucide-react';

const NFTCard = ({ imageUrl, title, price, lastSold = true }) => (
  <div className="bg-[#1E1E1E] rounded-xl overflow-hidden hover:ring-2 hover:ring-gray-600 transition-all cursor-pointer">
    <div className="aspect-square w-full overflow-hidden">
      <img 
        src={imageUrl} 
        alt={title}
        className="w-full h-full object-cover transform hover:scale-105 transition-transform"
      />
    </div>
    <div className="p-4">
      <h3 className="text-gray-200 text-sm mb-2">{title}</h3>
      <div className="flex justify-between items-center">
        <span className="text-white font-semibold">${price}</span>
        {lastSold && (
          <span className="text-gray-500 text-sm">LAST SOLD</span>
        )}
      </div>
    </div>
  </div>
);

const ValueFilter = () => (
  <button className="flex items-center gap-2 bg-[#1E1E1E] px-4 py-2 rounded-lg text-gray-300 hover:bg-[#2A2A2A] transition-colors">
    Value: High to low
    <ChevronDown className="w-4 h-4" />
  </button>
);

const NFTShowcase = () => {
  const bestProducts = [
    {
      imageUrl: "https://storage.googleapis.com/nftimagebucket/base/tokens/0x678f747d90afc7617d6b0b88a96a70d8785712e7/preview/TVRjeU5qazJOVFl6TVE9PV8xNDQ0.webp",
      title: "Gremlin Base Boys #1444",
      price: "0.9 ETH"
    },
    {
      imageUrl: "https://storage.googleapis.com/nftimagebucket/base/tokens/0x678f747d90afc7617d6b0b88a96a70d8785712e7/preview/TVRjeU5qazJOVFl6TVE9PV8xNDQz.webp",
      title: "Gremlin Base Boys #1443",
      price: "0.9 ETH"
    },
    {
      imageUrl: "https://storage.googleapis.com/nftimagebucket/base/tokens/0x7860bd7386da0ed83fb939b16097cece8dc95609/preview/TVRjeE1qVTVNelV3Tmc9PV81MTE=.gif",
      title: "Chimkens #511",
      price: "0.5 ETH"
    },
    {
      imageUrl: "https://storage.googleapis.com/nftimagebucket/base/tokens/0xbe3c7abab76f0a1de602fdb2f44faf604a5f649f/preview/TVRjeU5UY3hNRFF4TkE9PV8zMA==.png",
      title: "Unrevealed Hooman",
      price: "0.1 ETH"
    },
  ];

  const collection = [
    {
      imageUrl: "https://storage.googleapis.com/nftimagebucket/base/tokens/0x678f747d90afc7617d6b0b88a96a70d8785712e7/preview/TVRjeU5qazJOVFl6TVE9PV8xNDQy.webp",
      title: "Gremlin Base Boys #1442",
      price: "0.9 ETH"
    },
    {
      imageUrl: "https://storage.googleapis.com/nftimagebucket/base/tokens/0x3eb30a305f6c5e04c9206064be6b84f870e060bc/preview/TVRjeE1qWXdNVFE1TVE9PV81MDM=.png",
      title: "Trailer Park Pups #503",
      price: "0.6 ETH"
    },
    {
      imageUrl: "https://storage.googleapis.com/nftimagebucket/base/tokens/0x3eb30a305f6c5e04c9206064be6b84f870e060bc/preview/TVRjeE1qWXdNVFE1TVE9PV81MDc=.png",
      title: "Trailer Park Pups #507",
      price: "0.6 ETH"
    },
    {
      imageUrl: "https://storage.googleapis.com/nftimagebucket/base/tokens/0x3eb30a305f6c5e04c9206064be6b84f870e060bc/preview/TVRjeE1qWXdNVFE1TVE9PV81MDQ=.png",
      title: "Trailer Park Pups #504",
      price: "0.6 ETH"
    }
  ];

  return (
    <div className="min-h-screen bg-[#131313] p-6 mt-[60px]">
      <div className="max-w-7xl mx-auto space-y-12">
        <section>
          <div className="flex justify-between items-center mb-6">
            <h2 className="text-white text-2xl font-bold">NFTs</h2>
            <ValueFilter />
          </div>
          <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
            {bestProducts.map((nft, index) => (
              <NFTCard key={`best-${index}`} {...nft} />
            ))}
          </div>
        </section>

        {/* Collection Section */}
        <section>
          <div className="flex justify-between items-center mb-6">
            <h2 className="text-white text-2xl font-bold">Collection</h2>
            <ValueFilter />
          </div>
          <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
            {collection.map((nft, index) => (
              <NFTCard key={`collection-${index}`} {...nft} />
            ))}
          </div>
        </section>
      </div>
      <FloatingOverlay />
    </div>
  );
};

export default NFTShowcase;