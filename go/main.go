package main
import (
    "fmt"
    "time"
    "sync"
    "math/big"
    "crypto/ecdsa"
    "github.com/ethereum/go-ethereum/core/types"
    "github.com/ethereum/go-ethereum/common"
    "github.com/ethereum/go-ethereum/crypto"
    "github.com/miguelmota/go-solidity-sha3"
)

const REPEAT_COUNT = 100_000

func main() {
    var wg sync.WaitGroup
    wg.Add(REPEAT_COUNT)
    start := time.Now()
    signer, _ := getSigner()
    for i := 0; i < REPEAT_COUNT; i++ {
        go hash(signer, &wg)
    }
    wg.Wait()
    fmt.Println(time.Since(start).Milliseconds())
}

func hash(privateKey *ecdsa.PrivateKey, wg *sync.WaitGroup) string {
    defer wg.Done()
    hashData := solsha3.SoliditySHA3(
		[]string{"string", "uint256"},
		[]interface{}{
			"The quick brown fox jumps over the lazy dog",
			"1337",
		},
	)
    signature, _ := crypto.Sign(hashData, privateKey)
    nonce := uint64(0)
    chainID :=  big.NewInt(1)
    value := big.NewInt(0)
    gasLimit := uint64(21000)
    gasPrice := big.NewInt(0)
    toAddress :=
        common.HexToAddress("0xeC9D1C79A92A6c108b6aa9B101E86d58034843B5")
    tx := types.NewTransaction(
        nonce,
        toAddress,
        value,
        gasLimit,
        gasPrice,
        signature,
    )
    txSigned, _ := types.SignTx(tx, types.NewEIP155Signer(chainID), privateKey)
    txHashHex := txSigned.Hash().Hex()
    return txHashHex
}

func getSigner() (*ecdsa.PrivateKey, error) {
    hexPrivateKey :=
        "0xbeefbeefbeefbeefbeefbeefbeefbeefbeefbeefbeefbeefbeefbeefbeefbeef"
    privateKey, err := crypto.HexToECDSA(hexPrivateKey[2:])
    return privateKey, err
}
