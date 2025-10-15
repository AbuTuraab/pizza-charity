import { contracts } from "@polkadot-api/descriptors"
import * as pizzaPassethub from "contracts/deployments/pizza_limited/passethub"



export const pizza = {
  contract: contracts.pizza_limited,
  evmAddresses: {
    passethub: pizzaPassethub.evmAddress,
    // Add more deployments here
  },
  ss58Addresses: {
    passethub: pizzaPassethub.ss58Address,
    // Add more deployments here
  },
}

export const deployments = {
  pizza
  // Add more contracts here
}
