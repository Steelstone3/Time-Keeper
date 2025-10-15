# To cross compile windows mingw64-gcc-c++/ mingw-w64
# cargo build --target=x86_64-pc-windows-gnu --release

#!/bin/bash

if [ -f ./target/release/time-keeper ]; then
    echo "Executable found: ./target/release/time-keeper. Running it directly."
    ./target/release/time-keeper
else
    read -p "Get latest pre-compiled release? (Y/n) " choice
    choice=${choice:-Y}  # Default to "Y" if user presses Enter
    
    if [[ "$choice" =~ ^[Yy]$ ]]; then
        echo Downloading latest release
        curl -s https://api.github.com/repos/Steelstone3/Time-Keeper/releases/latest \
        | grep "browser_download_url.*x86_64-unknown-linux-gnu.tar.gz" \
        | cut -d '"' -f 4 \
        | xargs curl -LO
        
        echo Extracting executable binary
        tar -xzf time-keeper-*-x86_64-unknown-linux-gnu.tar.gz
        
        echo Makinging directory ./target/release
        mkdir -p ./target/release
        
        echo Moving executable binary
        mv time-keeper ./target/release/
        
        echo Cleaning up tarball
        rm time-keeper-*-x86_64-unknown-linux-gnu.tar.gz
        
        ./target/release/time-keeper
    else
        cargo build --release
        
        ./target/release/time-keeper
    fi
fi
