curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
sudo apt install mysql-server mysql-client libmysqlclient-dev
sudo systemctl start mysql
sudo systemctl enable mysql

# Create .env file with your configuration

# Install sqlx-cli for migrations
cargo install sqlx-cli --features mysql

# Run migrations
sqlx migrate run --database-url "mysql://portfolio_user:password@localhost/portfolio"

# Build and run
cargo run

# Production
cargo build --release
./target/release/portfolio-backend

# To run it as a service
- Create portfolio-backend.service file into /lib/systemd/system
- Move executable into folder that is noted in service file alongside of static, templates, uploads and .env file

sudo systemctl daemon-reload
sudo systemctl start portfolio-backend
sudo systemctl enable portfolio-backend
