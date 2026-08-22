ExUnit.start()

defmodule Wasm4pmCompat.AshTypesTest do
  use ExUnit.Case, async: true

  alias Wasm4pmCompat.AshTypes

  test "every exported projection is a real Ash type with stable source identity" do
    descriptors = AshTypes.descriptors()

    assert length(descriptors) == 71
    assert length(Enum.uniq_by(descriptors, & &1.short_name)) == length(descriptors)
    assert length(Enum.uniq_by(descriptors, & &1.module)) == length(descriptors)

    for descriptor <- descriptors do
      assert Ash.Type.ash_type?(descriptor.module)
      assert descriptor.module.source_rust_type() == descriptor.source_rust_type
      assert descriptor.module.projection_class() == descriptor.projection
      assert descriptor.module.source_feature() == descriptor.source_feature
      assert AshTypes.fetch!(descriptor.short_name) == descriptor.module
    end
  end

  test "map projections cast usable structural values" do
    type = Wasm4pmCompat.AshTypes.Ocel.OcelLog
    value = %{"events" => [], "objects" => []}

    assert {:ok, ^value} = Ash.Type.cast_input(type, value, [])
  end

  test "scalar projections cast through their admitted Ash subtype" do
    assert {:ok, 42} =
             Ash.Type.cast_input(Wasm4pmCompat.AshTypes.Ids.EventId, 42, [])

    assert {:ok, :error} =
             Ash.Type.cast_input(
               Wasm4pmCompat.AshTypes.Diagnostic.DiagnosticSeverity,
               :error,
               []
             )
  end

  test "opaque projections remain usable without inventing stronger semantics" do
    type = Wasm4pmCompat.AshTypes.Conformance.Fitness
    value = %{numerator: 97, denominator: 100}

    assert {:ok, ^value} = Ash.Type.cast_input(type, value, [])
  end

  test "arrays preserve Ash's native array type constructor" do
    assert AshTypes.array(:wasm4pm_ocel_log) ==
             {:array, Wasm4pmCompat.AshTypes.Ocel.OcelLog}
  end

  test "custom type registration surface is complete" do
    custom_types = AshTypes.custom_types()

    assert length(custom_types) == 71
    assert Keyword.fetch!(custom_types, :wasm4pm_event_log) ==
             Wasm4pmCompat.AshTypes.Eventlog.EventLog
  end
end
