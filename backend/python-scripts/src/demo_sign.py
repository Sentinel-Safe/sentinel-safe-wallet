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
    """Create a new transaction proposal interactively"""
    print("\n" + "=" * 60)
    print("💼 Create New Transaction")
    print("=" * 60)

    # Get recipient address
    print("\n📮 Enter recipient address")
    print("   (Press Enter for default: 0x6f512E3F002065813B92009C74E3a7966e7F87E1)")
    print("   > ", end="")
    to_address = input().strip()
    if not to_address:
        to_address = "0x6f512E3F002065813B92009C74E3a7966e7F87E1"

    # Get amount
    print("\n💰 Enter amount of KAIA to send")
    print("   (Press Enter for default: 0.001 KAIA)")
    print("   > ", end="")
    amount_str = input().strip()
    if not amount_str:
        amount_str = "0.001"

    # Convert to wei
    amount_wei = str(int(float(amount_str) * 10**18))

    print("\n📝 Creating transaction...")
    print(f"   To: {to_address}")
    print(f"   Amount: {amount_str} KAIA")

    response = requests.post(
        f"{API_URL}/transactions",
        json={
            "to": to_address,
            "value": amount_wei,
            "data": None,
        },
    )

    if response.status_code != 200:
        raise Exception(f"Failed to create transaction: {response.text}")

    result = response.json()
    print("\n✅ Transaction created successfully!")
    print(f"   TX ID: {result['tx_id']}")
    print(f"   Safe TX Hash: {result['safe_tx_hash'][:10]}...")

    return result


def sign_message_with_key(message_hash: str, private_key: str) -> str:
    """Sign a message hash with a private key for Safe transactions"""
    from eth_utils import keccak
    from web3 import Web3

    # Debug: Print the hash we received
    print(
        f"   Debug - Received hash: {message_hash[:70]}... (length: {len(message_hash)})"
    )

    # Safe transaction hash should be exactly 66 characters (0x + 64 hex chars)
    if len(message_hash) == 66 and message_hash.startswith("0x"):
        # This is the correct format - a 32-byte hash
        message_bytes = bytes.fromhex(message_hash[2:])
    else:
        # Something is wrong - log for debugging
        print(
            f"   WARNING: Unexpected hash format! Expected 66 chars, got {len(message_hash)}"
        )
        # For now, handle it as before
        if len(message_hash) > 66:
            # This shouldn't happen for Safe tx hash
            message_bytes = keccak(
                message_hash.encode() if isinstance(message_hash, str) else message_hash
            )
        else:
            if not message_hash.startswith("0x"):
                message_hash = "0x" + message_hash
            if len(message_hash) < 66:
                message_hash = "0x" + message_hash[2:].zfill(64)
            message_bytes = bytes.fromhex(message_hash[2:])

    if len(message_bytes) != 32:
        raise ValueError(f"Hash must be exactly 32 bytes, got {len(message_bytes)}")

    # Use eth_account's signing method
    w3 = Web3()
    account = w3.eth.account.from_key(private_key)

    # Sign the hash directly (without ethereum message prefix)
    from eth_account import Account

    # Create the signature
    signature = Account._sign_hash(message_bytes, private_key)

    # Return the signature for Safe (r + s + v format)
    # Safe expects v to be 27 or 28 for EOA signatures
    sig_hex = (
        signature.r.to_bytes(32, "big")
        + signature.s.to_bytes(32, "big")
        + bytes([signature.v])
    )

    return "0x" + sig_hex.hex()


def submit_signature(
    tx_id: str,
    signer_name: str,
    signer_info: Dict[str, str],
    tx_hash: str,
    signatures_count: int,
) -> bool:
    """Submit a signature to the orchestrator with interactive confirmation"""

    print(f"\n{'=' * 50}")
    print(f"🔐 {signer_name} ({signer_info['type'].replace('_', ' ').title()})")
    print(f"{'=' * 50}")
    print(f"📍 Address: {signer_info['address']}")
    print(f"📊 Current signatures: {signatures_count}/5")

    # AI agents can analyze the transaction
    if signer_info["type"] == "ai_agent":
        print(f"\n🤖 {signer_name} analyzing...")
        time.sleep(0.5)  # Simulate analysis time

        if "CFO" in signer_name:
            print("   💰 Financial rules verification: ✅ 1 KAIA - within daily limit")
            print("   📊 Budget compliance: ✅ Within test limits")
        elif "Security" in signer_name:
            print("   🔒 Recipient address verification: ✅ Not blacklisted")
            print("   ⚠️  Risk assessment: Low")
        elif "Analyst" in signer_name:
            print("   📈 Transaction analysis: Simple transfer")
            print("   🔍 Contract risk: None")

    # Ask for confirmation
    print(f"\n❓ Sign with {signer_name}? (y/n): ", end="")
    response = input().strip().lower()

    if response != "y":
        print(f"   ⏭️  Skipped {signer_name} signature")
        print()
        return False

    print("\n🖊️  Signing...")
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
                f"   ✅ Signature complete! (Total {result['current_signatures']}/{result['required_signatures']} signatures collected)"
            )
            return True
        else:
            print(f"   ❌ Signature failed: {result.get('error', 'Unknown error')}")
    else:
        print(f"   ❌ Request failed: {response.status_code}")

    return False


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

        # 2. Collect signatures from all signers interactively
        signers_list = list(DEMO_SIGNERS.items())

        if len(signers_list) < 4:
            print(
                f"❌ Error: Need at least 4 signers, but only {len(signers_list)} found"
            )
            return 1

        print("\n" + "=" * 60)
        print("📝 Starting signature collection (minimum 4/5 required)")
        print("=" * 60)

        signatures_collected = 0
        signed_addresses = set()

        # Go through all 5 signers
        for signer_name, signer_info in signers_list:
            # Skip if already signed
            if signer_info["address"] in signed_addresses:
                continue

            # Submit signature with current count
            if submit_signature(
                tx_id, signer_name, signer_info, tx_hash, signatures_collected
            ):
                signatures_collected += 1
                signed_addresses.add(signer_info["address"])

                # Check if we have enough signatures
                if signatures_collected >= 4:
                    print(f"\n🎉 Sufficient signatures collected! ({signatures_collected}/5)")

                    # Ask if they want to continue with the 5th signer
                    if (
                        signatures_collected < 5
                        and (
                            len(signers_list)
                            - signers_list.index((signer_name, signer_info))
                            - 1
                        )
                        > 0
                    ):
                        print("\n❓ Collect additional signatures? (y/n): ", end="")
                        continue_signing = input().strip().lower()
                        if continue_signing != "y":
                            break

        print("\n" + "=" * 60)
        # 3. Check final status
        status = check_status(tx_id)

        # 4. Execute transaction if we have enough signatures
        if status and status["signatures_collected"] >= 4:
            print("\n❓ Execute the transaction? (y/n): ", end="")
            execute_response = input().strip().lower()

            if execute_response == "y":
                execute_result = execute_transaction(tx_id)
            else:
                print("⏸️  Transaction execution deferred")
        else:
            print(
                f"\n❌ Cannot execute due to insufficient signatures ({status['signatures_collected']}/4 required)"
            )

        print()
        print("=" * 60)
        print("🏁 Demo completed!")
        print("=" * 60)
        print()

    except Exception as e:
        print(f"❌ Error: {e}")
        return 1

    return 0


if __name__ == "__main__":
    exit(main())
