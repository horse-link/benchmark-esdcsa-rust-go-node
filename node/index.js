const ethers = require("ethers");

const REPEAT_COUNT = 100_000;

const main = () => Promise.all([...Array(REPEAT_COUNT)].map(_ => hash()));

const hash = async () => {
  const message = ethers.utils.arrayify(ethers.utils.solidityKeccak256(
    ["string", "uint256"],
    [
      "The quick brown fox jumps over the lazy dog",
      1337
    ]
  ));
  const signer = getSigner();
  const signature = await signer.signMessage(message);
  const tx = {
    from: "0xeC9D1C79A92A6c108b6aa9B101E86d58034843B5",
    to: "0xeC9D1C79A92A6c108b6aa9B101E86d58034843B5",
    value: 0,
    data: signature
  };
  await signer.signTransaction(tx);
};

const getSigner = () => new ethers.Wallet(
  "beefbeefbeefbeefbeefbeefbeefbeefbeefbeefbeefbeefbeefbeefbeefbeef"
);

const start = Date.now();
main().then(() => {
    const duration = Date.now() - start;
    console.log(`execution took ${duration} milliseconds`);
});
