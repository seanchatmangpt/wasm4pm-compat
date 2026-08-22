defmodule Wasm4pmCompatAshTypes.MixProject do
  use Mix.Project

  def project do
    [
      app: :wasm4pm_compat_ash_types,
      version: "26.8.7",
      elixir: "~> 1.17",
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  def application do
    [extra_applications: [:logger]]
  end

  defp deps do
    [
      {:ash, "~> 3.32.0"}
    ]
  end
end
