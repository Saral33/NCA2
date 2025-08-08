# Nepal Christian App (NCA2)

A modern desktop application for the Nepali Christian community, built with Tauri, React, and TypeScript that provides access to Nepali Christian hymns (Bhajans) and the complete Bible in Nepali language. This is upgrade from old electron based app [NCA](https://github.com/Saral33/NCA)

## 🌟 Features

### 🎵 Bhajan Kitab (Hymn Book)

- **Complete Nepali Hymn Collection**: Access to a comprehensive collection of Nepali Christian hymns
- **Multiple Categories**:
  - Bhajans (B) - Main hymns
  - Chorus (C) - Chorus sections
  - Bal Chorus (BC) - Children's chorus
- **Easy Navigation**: Browse through hymns with smooth scrolling and intuitive interface
- **Detailed View**: Click on any hymn to view its complete lyrics and details

### 📖 Bible in Nepali

- **Complete Bible**: All 66 books of the Bible in Nepali language
- **Chapter Navigation**: Easy navigation between chapters with previous/next buttons
- **Verse-by-Verse Reading**: Clean, readable format with verse numbers
- **Book Selection**: Dropdown menu to select any book of the Bible
- **Responsive Design**: Optimized for desktop viewing with proper typography

### 🎨 User Interface

- **Dark Theme**: Modern dark interface for comfortable reading
- **Nepali Typography**: Custom Preeti font for authentic Nepali text rendering
- **Responsive Layout**: Optimized for desktop screens
- **Smooth Animations**: Enhanced user experience with smooth transitions
- **Intuitive Navigation**: Easy-to-use interface with clear navigation paths

## 🛠️ Tech Stack

- **Frontend**: React 18 + TypeScript + Vite
- **Backend**: Rust (Tauri)
- **Styling**: CSS with PostCSS
- **Build Tool**: Vite
- **Package Manager**: npm

## 📋 Prerequisites

Before running this application, make sure you have the following installed:

- [Node.js](https://nodejs.org/) (v16 or higher)
- [Rust](https://rustup.rs/) (latest stable version)
- [Tauri prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites) for your operating system

## 🔧 Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/yourusername/NCA2.git
   cd NCA2
   ```

2. **Install frontend dependencies**
   ```bash
   npm install
   ```

3. **Install Rust dependencies** (handled automatically by Tauri)

## 🚀 Development

To start the development server:

```bash
npm run tauri dev
```

This will:
- Start the Vite development server on `http://localhost:5173`
- Launch the Tauri application window
- Enable hot-reload for both frontend and backend changes

## 🏗️ Building

To build the application for production:

```bash
npm run tauri build
```

This will create:
- Platform-specific installers in `src-tauri/target/release/bundle/`
- Optimized frontend assets
- Native executable files

## 📁 Project Structure

```
NCA2/
├── src/                    # React frontend source
│   ├── App.tsx            # Main React component
│   ├── main.tsx           # React entry point
│   └── assets/            # Static assets
├── src-tauri/             # Tauri backend
│   ├── src/               # Rust source code
│   ├── Cargo.toml         # Rust dependencies
│   ├── tauri.conf.json    # Tauri configuration
│   └── icons/             # Application icons
├── public/                # Public assets
├── package.json           # Node.js dependencies
├── vite.config.ts         # Vite configuration
└── tsconfig.json          # TypeScript configuration
```

## 🎯 Key Features Explained

### Home Screen

- Clean, minimalist interface with two main options
- Bhajan Kitab (Hymn Book) - Access to Nepali hymns
- Bible - Complete Bible in Nepali

### Bhajan Screen

- Three categories: Bhajans (B), Chorus (C), Bal Chorus (BC)
- Virtual scrolling for smooth performance with large datasets
- Click any hymn to view detailed lyrics
- Search and navigation controls

### Bible Screen

- Complete Bible with all 66 books
- Chapter-by-chapter navigation
- Verse-by-verse display with proper formatting
- Easy book selection dropdown
- Previous/Next chapter navigation

## ⚙️ Configuration

### Application Settings

The app configuration is defined in [`src-tauri/tauri.conf.json`](src-tauri/tauri.conf.json):

- **Product Name**: Nepali Christiam App
- **Version**: 0.1.0
- **Bundle Identifier**: com.saralkarki.nca2
- **Window Size**: 1100x800 (minimum 800x600)

### Development URLs

- **Development Server**: http://localhost:5173
- **Frontend Build Output**: `dist/`

## 🎯 Available Scripts

| Script | Description |
|--------|-------------|
| `npm run dev` | Start Vite development server |
| `npm run build` | Build frontend for production |
| `npm run tauri dev` | Start Tauri development mode |
| `npm run tauri build` | Build complete application |


## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the [MIT License](LICENSE).

## 👥 Acknowledgments

- Built with [Tauri](https://tauri.app/) for cross-platform desktop development
- Frontend powered by [React](https://reactjs.org/) and [Vite](https://vitejs.dev/)
- Icons and assets for the Nepali Christian community

## 📞 Support

For support, please open an issue in the GitHub repository or contact the development team.

---

Made with ❤️ for the Nepali Christian community

