# 🚦 Traffic Intersection Simulation

A realistic 4-way traffic intersection simulator built with Rust and SDL2. Features intelligent traffic light management, collision prevention, and multiple vehicle routing options.

## 🎯 Features

### 🚗 Vehicle System
- **Multiple Routes**: Vehicles can go straight, turn left, or turn right
- **Color-Coded Routes**: 
  - 🟡 **Yellow**: Left turns
  - 🔵 **Blue**: Right turns  
  - ⚫ **Grey**: Straight through
- **Realistic Movement**: Vehicles follow proper turning paths through the intersection
- **Safety Distance**: Automatic collision prevention with 60-pixel following distance
- **Lane Awareness**: Vehicles stay in their designated lanes

### 🚦 Traffic Light System
- **Sequential Direction Control**: Each direction gets exclusive access to prevent all collisions
- **Smart Phase Management**: 
  - Phase 1: UP traffic only (from south)
  - Phase 2: DOWN traffic only (from north)
  - Phase 3: LEFT traffic only (from east)
  - Phase 4: RIGHT traffic only (from west)
- **Intersection Clearance**: Extends red lights if vehicles are still clearing the intersection

## 🎮 Controls

| Key | Action |
|-----|--------|
| `↑` | Spawn vehicle from **South** (going UP) |
| `↓` | Spawn vehicle from **North** (going DOWN) |
| `←` | Spawn vehicle from **East** (going LEFT) |
| `→` | Spawn vehicle from **West** (going RIGHT) |
| `R` | Spawn vehicle from **random direction** |
| `ESC` | Exit simulation |

## 🛠️ Installation & Setup

### Prerequisites
- Rust (2021 edition or later)
- SDL2 development libraries
- SDL2_ttf for font rendering

### Platform-Specific SDL2 Setup

#### Ubuntu/Debian
```bash
sudo apt-get install libsdl2-dev libsdl2-ttf-dev
```

#### macOS (with Homebrew)
```bash
brew install sdl2 sdl2_ttf
```

#### Windows
- Download SDL2 development libraries from [libsdl.org](https://www.libsdl.org/)
- Follow the Rust-SDL2 [installation guide](https://github.com/Rust-SDL2/rust-sdl2#windows-msvc)

### Running the Project
```bash
git clone [<repository-url>](https://github.com/achnitreda/road_intersection.git)
cd road_intersection
cargo run
```

## 🏗️ Project Structure

```
road_intersection/
├── src/
│   └── main.rs          # Main simulation code
├── Cargo.toml           # Dependencies and project config
├── Cargo.lock           # Dependency lock file
└── README.md           # Readme
```

## 📐 Technical Details

### Coordinate System
- **Screen Size**: 1000 x 800 pixels
- **Intersection**: 425-575 (X) × 325-475 (Y)
- **Lane Width**: ~75 pixels
- **Vehicle Size**: 50×50 pixels

### Spawn Positions
| Direction | Position | Lane |
|-----------|----------|------|
| UP (↑) | (515, 700) | Right lane |
| DOWN (↓) | (440, 0) | Left lane |
| LEFT (←) | (950, 335) | Top lane |
| RIGHT (→) | (10, 415) | Bottom lane |

### Performance
- **Frame Rate**: 60 FPS
- **Memory**: Minimal - vehicles removed when off-screen

## 🐛 Troubleshooting

### Common Issues

**SDL2 Not Found**
```
error: failed to run custom build command for `sdl2-sys`
```
**Solution**: Install SDL2 development libraries (see Installation section)

**Font Loading Error**
```
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value'
```
**Solution**: The program tries multiple font paths. Ensure you have system fonts installed.

**Poor Performance**
- Reduce vehicle count
- Check if running in debug mode (try `cargo run --release`)

## 📜 License

This project is open source. Feel free to use, modify, and distribute.

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## 📞 Contact

For questions, issues, or suggestions, please open an issue in the repository.

---

**Built with ❤️ using Rust and SDL2**
