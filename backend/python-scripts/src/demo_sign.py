#!/usr/bin/env python3

"""
Demo script showing how 5 signers (2 humans + 3 AI agents) would sign a transaction
with their private keys and submit to the orchestrator.

In production:
- Human signers would use MetaMask or hardware wallets
- AI agents would be separate services with secure key management
- Private keys would NEVER be in the same place
"""

import os
import time
from typing import Any, Dict

import requests
from dotenv import load_dotenv
from eth_account import Account

# Load environment variables from .env file
load_dotenv()


# Load private keys from environment variables
def load_signers() -> Dict[str, Dict[str, str]]:
    """Load signer information from environment variables"""
    signers = {}

    # Human signers
    if os.getenv("HUMAN1_PRIVATE_KEY"):
        account = Account.from_key(os.getenv("HUMAN1_PRIVATE_KEY"))
        signers["Human 1"] = {
            "address": account.address,
            "private_key": os.getenv("HUMAN1_PRIVATE_KEY"),
            "type": "human",
        }

    if os.getenv("HUMAN2_PRIVATE_KEY"):
        account = Account.from_key(os.getenv("HUMAN2_PRIVATE_KEY"))
        signers["Human 2"] = {
            "address": account.address,
            "private_key": os.getenv("HUMAN2_PRIVATE_KEY"),
            "type": "human",
        }

    # AI Agent signers
    if os.getenv("AI_CFO_PRIVATE_KEY"):
        account = Account.from_key(os.getenv("AI_CFO_PRIVATE_KEY"))
        signers["AI CFO"] = {
            "address": account.address,
            "private_key": os.getenv("AI_CFO_PRIVATE_KEY"),
            "type": "ai_agent",
        }

    if os.getenv("AI_SECURITY_PRIVATE_KEY"):
        account = Account.from_key(os.getenv("AI_SECURITY_PRIVATE_KEY"))
        signers["AI Security"] = {
            "address": account.address,
            "private_key": os.getenv("AI_SECURITY_PRIVATE_KEY"),
            "type": "ai_agent",
        }

    if os.getenv("AI_ANALYST_PRIVATE_KEY"):
        account = Account.from_key(os.getenv("AI_ANALYST_PRIVATE_KEY"))
        signers["AI Analyst"] = {
            "address": account.address,
            "private_key": os.getenv("AI_ANALYST_PRIVATE_KEY"),
            "type": "ai_agent",
        }

    return signers


# Load signers from environment
DEMO_SIGNERS = load_signers()

API_URL = "http://localhost:3001/api/v1"


def create_transaction() -> Dict[str, Any]:
    """Create a new transaction proposal"""
    print("📝 Creating transaction proposal...")

    response = requests.post(
        f"{API_URL}/transactions",
        json={
            "to": "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb5",
            "value": "1000000000000000",  # 0.001 ETH
            "data": None,
        },
    )

    if response.status_code != 200:
        raise Exception(f"Failed to create transaction: {response.text}")

    result = response.json()
    print("✅ Transaction created:")
    print(f"   TX ID: {result['tx_id']}")
    print(f"   Hash to sign: {result['safe_tx_hash']}")
    print()

    return result


def sign_message_with_key(message_hash: str, private_key: str) -> str:
    """Sign a message hash with a private key using EIP-191"""
    # Remove 0x prefix if present
    if message_hash.startswith("0x"):
        message_hash = message_hash[2:]

    # Convert hash to bytes
    message_bytes = bytes.fromhex(message_hash)

    # Create account from private key
    account = Account.from_key(private_key)

    # For Safe transactions, we sign the hash directly (not as an Ethereum message)
    # This is equivalent to web3.eth.account.signHash()
    signature = account.signHash(message_bytes)

    # Return signature in hex format (65 bytes: r[32] + s[32] + v[1])
    return "0x" + signature.signature.hex()


def submit_signature(
    tx_id: str, signer_name: str, signer_info: Dict[str, str], tx_hash: str
):
    """Submit a signature to the orchestrator"""

    print(f"🖊️  {signer_name} signing...")
    print(f"   Address: {signer_info['address']}")
    print(f"   Type: {signer_info['type']}")

    # All signers provide their own signatures
    signature = sign_message_with_key(tx_hash, signer_info["private_key"])

    response = requests.post(
        f"{API_URL}/transactions/{tx_id}/sign",
        json={"signer_address": signer_info["address"], "signature": signature},
    )

    if response.status_code == 200:
        result = response.json()
        if "success" in result and result["success"]:
            print(
                f"   ✅ Signed successfully ({result['current_signatures']}/{result['required_signatures']} signatures)"
            )
        else:
            print(f"   ❌ Signing failed: {result.get('error', 'Unknown error')}")
    else:
        print(f"   ❌ Request failed: {response.status_code}")

    print()
    return response.json()


def check_status(tx_id: str):
    """Check transaction status"""
    response = requests.get(f"{API_URL}/transactions/{tx_id}/status")
    if response.status_code == 200:
        status = response.json()
        print("📊 Transaction Status:")
        print(
            f"   Signatures collected: {status['signatures_collected']}/{status['required_signatures']}"
        )
        print(f"   Status: {status['status']}")

        if status["signers"]:
            print("   Signers:")
            for signer in status["signers"]:
                print(f"     - {signer['address']} ({signer['type']})")
        print()
        return status
    return None


def execute_transaction(tx_id: str):
    """Execute the transaction after collecting enough signatures"""
    print("🚀 Executing transaction...")

    response = requests.post(f"{API_URL}/transactions/{tx_id}/execute")
    if response.status_code == 200:
        result = response.json()
        if result["success"]:
            print("✅ Transaction executed successfully!")
            print(f"   Transaction hash: {result['tx_hash']}")
        else:
            print("❌ Transaction execution failed")
    else:
        print(f"❌ Request failed: {response.status_code}")

    return response.json()


def main():
    """Main demo flow"""
    print("=" * 60)
    print("🔐 Sentinel Safe Wallet - Private Key Signing Demo")
    print("=" * 60)
    print()

    # Check if keys are loaded
    if not DEMO_SIGNERS:
        print("❌ Error: No private keys found in environment variables!")
        print()
        print("Please create a .env file with the following keys:")
        print("  HUMAN1_PRIVATE_KEY=0x...")
        print("  HUMAN2_PRIVATE_KEY=0x...")
        print("  AI_CFO_PRIVATE_KEY=0x...")
        print("  AI_SECURITY_PRIVATE_KEY=0x...")
        print("  AI_ANALYST_PRIVATE_KEY=0x...")
        print()
        print("For testing, you can use test keys from Hardhat/Anvil")
        return 1

    print(f"✅ Loaded {len(DEMO_SIGNERS)} signers from environment")
    for name, info in DEMO_SIGNERS.items():
        print(f"   - {name}: {info['address']}")
    print()

    # Note about security
    print("⚠️  SECURITY NOTE:")
    print("This demo uses test private keys for demonstration only.")
    print("In production:")
    print("- Private keys would NEVER be in the same place")
    print("- Humans would use MetaMask or hardware wallets")
    print("- AI agents would run as separate secure services")
    print("=" * 60)
    print()

    try:
        # 1. Create transaction
        tx_result = create_transaction()
        tx_id = tx_result["tx_id"]
        tx_hash = tx_result["safe_tx_hash"]

        # 2. Collect signatures from first 4 signers
        # This simulates the minimum required signatures
        signers_list = list(DEMO_SIGNERS.items())

        if len(signers_list) < 4:
            print(
                f"❌ Error: Need at least 4 signers, but only {len(signers_list)} found"
            )
            return 1

        # Use first 4 signers for the demo
        signers_to_use = signers_list[:4]

        print("📝 Collecting signatures (need 4 out of 5)...")
        print("-" * 40)
        print()

        for signer_name, signer_info in signers_to_use:
            submit_signature(tx_id, signer_name, signer_info, tx_hash)
            time.sleep(1)  # Small delay for demo effect

        # 3. Check status
        status = check_status(tx_id)

        # 4. Execute transaction if we have enough signatures
        if status and status["signatures_collected"] >= 4:
            print("✅ Sufficient signatures collected!")
            print()
            execute_result = execute_transaction(tx_id)
        else:
            print("❌ Not enough signatures to execute")

        print()
        print("=" * 60)
        print("Demo completed!")
        print()

        # Optional: Show what happens if 5th signer tries to sign
        print("📌 Optional: 5th signer (AI Analyst) could also sign...")
        print("   But transaction only needs 4 signatures")
        print()

    except Exception as e:
        print(f"❌ Error: {e}")
        return 1

    return 0


if __name__ == "__main__":
    exit(main())
