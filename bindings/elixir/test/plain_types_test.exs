defmodule Wasm4pmCompat.PlainTypesTest do
  use ExUnit.Case, async: true

  alias Wasm4pmCompat.PlainTypes

  test "every exported projection is a plain struct module with stable source identity" do
    descriptors = PlainTypes.descriptors()

    assert length(descriptors) == 71
    assert length(Enum.uniq_by(descriptors, & &1.short_name)) == length(descriptors)
    assert length(Enum.uniq_by(descriptors, & &1.module)) == length(descriptors)

    for descriptor <- descriptors do
      assert descriptor.module.source_rust_type() == descriptor.source_rust_type
      assert descriptor.module.subtype_of() == descriptor.subtype
      assert descriptor.module.projection_class() == descriptor.projection
      assert descriptor.module.source_feature() == descriptor.source_feature
      assert PlainTypes.fetch!(descriptor.short_name) == descriptor.module
    end
  end

  test "modules carry no dependency on Ash" do
    for descriptor <- PlainTypes.descriptors() do
      refute function_exported?(descriptor.module, :cast_input, 2)
      refute {:behaviour, [Ash.Type]} in descriptor.module.module_info(:attributes)
    end
  end

  test "new/1 and value/1 round-trip a structural map value" do
    type = Wasm4pmCompat.PlainTypes.Ocel.OcelLog
    value = %{"events" => [], "objects" => []}

    wrapped = type.new(value)

    assert %^type{value: ^value} = wrapped
    assert type.value(wrapped) == value
  end

  test "new/1 and value/1 round-trip a scalar value" do
    type = Wasm4pmCompat.PlainTypes.Ids.EventId

    assert type.new(42) |> type.value() == 42
  end

  test "custom type registration surface is complete and disjoint from AshTypes modules" do
    custom_types = PlainTypes.custom_types()

    assert length(custom_types) == 71

    assert Keyword.fetch!(custom_types, :wasm4pm_event_log) ==
             Wasm4pmCompat.PlainTypes.Eventlog.EventLog

    ash_modules = MapSet.new(Wasm4pmCompat.AshTypes.custom_types(), fn {_, mod} -> mod end)
    plain_modules = MapSet.new(custom_types, fn {_, mod} -> mod end)

    assert MapSet.disjoint?(ash_modules, plain_modules)
  end
end
