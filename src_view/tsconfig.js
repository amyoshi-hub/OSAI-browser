{
  "compilerOptions": {
    "target": "ESNext",
    "useDefineForClassFields": true,
    "lib": ["DOM", "DOM.Iterable", "ESNext"],
    "allowJs": false,
    "skipLibCheck": true,
    "esModuleInterop": false, // これは true の方が良いことが多いですが、tauriテンプレートは false の場合も
    "allowSyntheticDefaultImports": true, // これも true の方が良いことが多い
    "strict": true,
    "forceConsistentCasingInFileNames": true,
    "module": "ESNext",
    "moduleResolution": "Node", // もしくは "Bundler"
    "resolveJsonModule": true,
    "isolatedModules": true,
    "noEmit": true,
    "jsx": "react-jsx",
    "paths": {
      "@/*": ["./src/*"] // もしエイリアスを使っているなら
    }
  },
  "include": ["src", "./*.ts", "./*.tsx"], // <-- App.tsx が含まれるパス
  "references": [{ "path": "./tsconfig.node.json" }]
}
