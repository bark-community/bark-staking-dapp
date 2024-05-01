import { useState } from "react";
import styles from "../styles/Home.module.css";

export default function Header() {
  const [isConnected, setIsConnected] = useState(false);

  const handleConnectDisconnect = () => {
    setIsConnected((prevIsConnected) => !prevIsConnected);
  };

  return (
    <header className={styles.header}>
      <section className={styles.header_logoSection}>
        <h1>Stake</h1>
      </section>
      <section className={styles.header_nav}>
        <button className={styles.buyButton}>
          <a
            href="https://jup.ag/swap/SOL-BARK"
            target="_blank"
            rel="noopener noreferrer"
          >
            Buy BARK on Jupiter
          </a>
        </button>
        <button
          className={styles.walletButton}
          onClick={handleConnectDisconnect}
        >
          {isConnected ? "Disconnect Wallet" : "Connect Wallet"}
        </button>
      </section>
      <hr className={styles.header_line} />
    </header>
  );
}
