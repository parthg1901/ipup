import { buildModule } from "@nomicfoundation/hardhat-ignition/modules";


const IPUPModule = buildModule("IPUPModule", (m) => {
  const ipup = m.contract("IPUP");

  return { ipup };
});

export default IPUPModule;
