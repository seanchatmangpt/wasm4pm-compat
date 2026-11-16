defmodule Wasm4pmCompatAshTypes.MixProject do
  use Mix.Project

  def project do
    [
      app: :wasm4pm_compat_ash_types,
      description: "ggen-projected Elixir value carriers for wasm4pm-compat: dependency-free PlainTypes structs and Ash.Type.NewType-backed AshTypes projections of the public process-evidence surface.",
      version: "26.10.12",
      elixir: "~> 1.17",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      package: package(),
      docs: docs()
    ]
  end

  def application do
    [extra_applications: [:logger]]
  end

  defp deps do
    [
      {:ash, "~> 3.32.0"},
      {:ex_doc, "~> 0.34", only: :dev, runtime: false}
    ]
  end

  defp package do
    [
      licenses: ["MIT", "Apache-2.0"],
      links: %{
        "GitHub" => "https://github.com/seanchatmangpt/wasm4pm-compat",
        "Changelog" => "https://github.com/seanchatmangpt/wasm4pm-compat/blob/main/CHANGELOG.md"
      },
      files: ~w(lib test mix.exs README.md CHANGELOG.md LICENSE-MIT LICENSE-APACHE)
    ]
  end

  defp docs do
    [
      main: "Wasm4pmCompat.PlainTypes",
      source_url: "https://github.com/seanchatmangpt/wasm4pm-compat",
      source_url_pattern:
        "https://github.com/seanchatmangpt/wasm4pm-compat/blob/main/bindings/elixir/lib/wasm4pm_compat/%{path}#L%{line}"
    ]
  end
end
