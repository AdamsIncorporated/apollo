import React, { useState } from 'react';

interface SlidingSidebarProps {
  isOpen: boolean;
  onClose: () => void;
}

const SlidingSidebar: React.FC<SlidingSidebarProps> = ({ isOpen, onClose }) => {
  return (
    <div
      className={`fixed top-0 left-0 w-80 h-screen bg-gray-100 p-5 shadow-lg transition-all duration-300 ease-in-out z-1000 ${
        isOpen ? 'translate-x-0' : '-translate-x-full'
      }`}
    >
      <button className="absolute top-2 right-2 text-2xl" onClick={onClose}>
        &times;
      </button>
      <h2 className="text-xl font-semibold mb-4">Navigation</h2>
      <div className="py-2 border-b border-gray-300 cursor-pointer">Home</div>
      <div className="py-2 border-b border-gray-300 cursor-pointer">About</div>
      <div className="py-2 border-b border-gray-300 cursor-pointer">Services</div>
      <div className="py-2 cursor-pointer">Contact</div>
    </div>
  );
};

const SlidingSidebarDemo: React.FC = () => {
  const [isSidebarOpen, setIsSidebarOpen] = useState(false);

  const toggleSidebar = () => {
    setIsSidebarOpen(!isSidebarOpen);
  };

  return (
    <div>
      <button
        className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
        onClick={toggleSidebar}
      >
        {isSidebarOpen ? 'Close Sidebar' : 'Open Sidebar'}
      </button>
      <SlidingSidebar isOpen={isSidebarOpen} onClose={toggleSidebar} />
      <div className="p-5">
        <p>Main Content. Click the button to toggle the sidebar.</p>
      </div>
    </div>
  );
};

export default SlidingSidebarDemo;