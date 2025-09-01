# Starknet Caller

A Rust application for interacting with Starknet contracts remotely.

## Features

- Starknet smart contract interaction
- Secure transaction signing
- Environment-based configuration
- Support for custom contract calls

## Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)
- [fly.io CLI](https://fly.io/docs/hands-on/install-flyctl/)
- A Starknet account
- Access to a Starknet RPC endpoint

## Environment Variables

The application requires the following environment variables:

```env
STARKNET_RPC_URL=your_rpc_endpoint_url
STARKNET_PRIVATE_KEY=your_private_key
STARKNET_ACCOUNT_ADDRESS=your_account_address
STARKNET_CONTRACT_ADDRESS=your_contract_address
```

## Local Development

1. Clone the repository
```bash
git clone <repository-url>
cd starknet_caller
```

2. Create a `.env` file with your configuration ||use the `.config.toml` file to configure the application ( example in repo ).
```bash
cp .env.example .env
# Edit .env with your values
```

3. Build the project
```bash
cargo build
```

4. Run locally
```bash
cargo run
```

## Deployment to fly.io

WIP

## Architecture

The application consists of several key components:

- `StarknetContext`: Manages Starknet connection details
- `starknet_call_context()`: Initializes the Starknet context from environment variables
- `starknet_account()`: Creates a single-owner account instance
- `starknet_call()`: Executes transactions on the Starknet blockchain

## Security Considerations

- Never commit your private keys or sensitive environment variables
- Use secure methods to manage secrets in production
- Regularly rotate your keys and credentials
- Monitor your application's activities

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

[MIT License](LICENSE)

## Support

For support, please open an issue in the repository or contact the maintainers.
