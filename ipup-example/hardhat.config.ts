import { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";

const config: HardhatUserConfig = {
  solidity: {
    version: "0.8.23",
    settings: {
      evmVersion: "shanghai",
    },
  },
  networks: {
    "filecoin-calibration": {
      url: "https://api.calibration.node.glif.io/rpc/v1",
      accounts: ["ADD YOUR PRIVATE KEY"]
    },
    "subnet": {
      url: "http://localhost:8545",
      accounts: ["ADD YOUR PRIVATE KEY"]
    }
  },
};

export default config;
