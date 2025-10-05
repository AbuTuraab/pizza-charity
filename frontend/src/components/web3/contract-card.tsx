import { createReviveSdk, type ReviveSdkTypedApi } from "@polkadot-api/sdk-ink"
import { useChainId, useTypedApi } from "@reactive-dot/react"
import { useCallback, useEffect, useState } from "react"
import { toast } from "sonner"
import { useSignerAndAddress } from "@/hooks/use-signer-and-address"
import { pizza } from "@/lib/inkathon/deployments"
import { CardSkeleton } from "../layout/skeletons"
import { Button } from "../ui/button-extended"
import { Card, CardHeader, CardTitle } from "../ui/card"
import { Table, TableBody, TableCell, TableRow } from "../ui/table"
import { ALICE } from "@/lib/inkathon/constants"

export function ContractCard() {
  const [queryIsLoading, setQueryIsLoading] = useState(true)

  const api = useTypedApi()
  const chain = useChainId()
  const { signer, signerAddress } = useSignerAndAddress()

  /**
   * Contract Read (Query)
   */
  const [dailySupply, setDailySupply] = useState<any>()
  const [remainingSupply, setRemainingSupply] = useState<any>()
  const [maxOrderPerUser, setMaxOrderPerUser] = useState<any>()
  const [quantityOrdered, setQuantityOrdered] = useState<number>(0)
  
  const queryContract = useCallback(async () => {
    setQueryIsLoading(true)
    try {
      if (!api || !chain) return

      // Create SDK & contract instance
      const sdk = createReviveSdk(api as ReviveSdkTypedApi, pizza.contract)
      const contract = sdk.getContract(pizza.evmAddresses[chain])

      // Option 1: Query storage directly
      const storageResult = await contract.getStorage().getRoot()

       const result = await contract.query("get_daily_supply", {
          origin: ALICE
        }); 
      const newDailySupplyState = result.success ? result.value.response : undefined
      setDailySupply(newDailySupplyState)
      console.log(dailySupply);

       const result1 = await contract.query("get_remaining_supply", {
          origin: ALICE
        }); 
      const newRemainingSupplyState = result1.success ? result1.value.response : undefined
      setRemainingSupply(newRemainingSupplyState)

       
      const newMaxOrderPerUserState = storageResult.success ? storageResult.value.max_order_per_user-1 : undefined
      setMaxOrderPerUser(newMaxOrderPerUserState )
      
    } catch (error) {
      console.error(error)
    } finally {
      setQueryIsLoading(false)
    }
  }, [api, chain])

  useEffect(() => {
    queryContract()
  }, [queryContract])

  /**
   * Contract Write (Transaction)
   */
  const orderPizza = useCallback(async () => {
    if (!api || !chain || !signer) return

    const sdk = createReviveSdk(api as ReviveSdkTypedApi, pizza.contract, )
    const contract = sdk.getContract(pizza.evmAddresses[chain])

    // Map account if not mapped
    const isMapped = await sdk.addressIsMapped(signerAddress)
    if (!isMapped) {
      toast.error("Account not mapped. Please map your account first.")
      return
    }

    // Send transaction
    const tx = contract
      .send("order_pizza_and_pay",
        {
          data: {
            quantity_ordered: quantityOrdered
          },
          origin: signerAddress
        },

      )
      .signAndSubmit(signer)
      .then((tx) => {
        queryContract()
        if (!tx.ok) throw new Error("Failed to send transaction here, please check it", { cause: tx.dispatchError })
      })

    toast.promise(tx, {
      loading: "Sending transaction...",
      success: "Successfully Ordered, your Pizza is on its way",
      error: "Failed to send transaction here too",
    })
  }, [signer, api, chain])

  if (queryIsLoading) return <CardSkeleton />

  return (
    <Card className="inkathon-card">
      <CardHeader className="relative">
        <CardTitle>Pizza Request Contract</CardTitle>

        <Button
          variant="default"
          size="sm"
          className="-top-2 absolute right-6"
          onClick={() => orderPizza()}
        >
        Request Pizza
        </Button>
      </CardHeader>

      <Table className="inkathon-card-table">
        <TableBody>
          <TableRow>
            <TableCell>Total Daily Supply</TableCell>
            <TableCell>
              {dailySupply}
            </TableCell>
          </TableRow>

            <TableRow>
            <TableCell>Remaining Supply</TableCell>
            <TableCell>
              {remainingSupply}
            </TableCell>
          </TableRow>

            <TableRow>
            <TableCell>Maximum Order Per User</TableCell>
            <TableCell>
              {maxOrderPerUser}
            </TableCell>
          </TableRow>


          <TableRow>
            <TableCell>Address</TableCell>
            <TableCell>{pizza.evmAddresses[chain]}</TableCell>
          </TableRow>

          <TableRow>
                <TableCell>Pizza Amount</TableCell>
                <TableCell>
                  <input type="number" max="6" 
                  placeholder="Amount" 
                   value={quantityOrdered}
                   onChange={e => setQuantityOrdered(Number(e.target.value))}
                  />
                </TableCell>

          </TableRow>

        
    
        </TableBody>
      </Table>
    </Card>
  )
}
