import './App.css'

function App() {
  return (
    <main className="app-shell">
      <section className="hero" aria-labelledby="hero-title">
        <p className="eyebrow">Plataforma empresarial multiempresa</p>

        <h1 id="hero-title">KORDENT ERP</h1>

        <p className="tagline">El núcleo que coordina tu empresa.</p>

        <p className="description">
          Una plataforma modular, global y preparada para operar desde la web y
          sin conexión mediante la aplicación de escritorio.
        </p>

        <p className="status" role="status">
          <span className="status-dot" aria-hidden="true" />
          Aplicación web en desarrollo
        </p>
      </section>
    </main>
  )
}

export default App
