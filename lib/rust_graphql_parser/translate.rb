# frozen_string_literal: true
require "graphql/c_parser"
require 'graphql'

module RustGraphqlParser
  include GraphQL::Language::Nodes

  def self.translate(node)
    case node.fetch(:node_type)
    when :document
      Document.new(definitions: node.fetch(:definitions).map{|x| translate(x)})
    when :query, :mutation
      selections = node.fetch(:selection_set).fetch(:items).map{|x| translate(x)}
      variables = node.fetch(:variable_definitions).map{|x| translate(x)}
      directives = node.fetch(:directives).map{|x| translate(x)}
      OperationDefinition.new(
        operation_type: node.fetch(:node_type).to_s,
        name: node[:name],
        selections:,
        variables:,
        directives:,
      )
    when :selection_set
      selections = node.fetch(:items).map{|x| translate(x)}
      OperationDefinition.new(operation_type: "query", name: node[:name], selections:, variables: [])
    when :fragment_definition
      selections = node.fetch(:selection_set).fetch(:items).map{|x| translate(x)}
      type_name = node.fetch(:type_condition).fetch(:on)
      FragmentDefinition.new(
        name: node.fetch(:name),
        line: node.fetch(:position).fetch(:line),
        col: node.fetch(:position).fetch(:column),
        type: TypeName.new(name: type_name),
        selections:,
        )
    when :field
      selections = node.fetch(:selection_set).fetch(:items).map{|x| translate(x)}
      arguments = node.fetch(:arguments).map{|x| translate(x)}
      directives = node.fetch(:directives).map{|x| translate(x)}
      Field.new(
        name: node.fetch(:name),
        line: node.fetch(:position).fetch(:line),
        col: node.fetch(:position).fetch(:column),
        selections: selections,
        arguments:,
        directives:,
      )
    when :directive
      arguments = node.fetch(:arguments).map{|x| translate(x)}
      Directive.new(
        name: node.fetch(:name),
        arguments:,
      )
    when :fragment_spread
      FragmentSpread.new(
        name: node.fetch(:fragment_name),
        line: node.fetch(:position).fetch(:line),
        col: node.fetch(:position).fetch(:column),
      )
    when :inline_fragment
      selections = node.fetch(:selection_set).fetch(:items).map{|x| translate(x)}
      directives = node.fetch(:directives).map{|x| translate(x)}
      type_name = node.fetch(:type_condition).fetch(:on)
      InlineFragment.new(
        line: node.fetch(:position).fetch(:line),
        col: node.fetch(:position).fetch(:column),
        type: TypeName.new(name: type_name),
        selections:,
        directives:,
      )
    when :variable_definition
      VariableDefinition.new(
        line: node.fetch(:position).fetch(:line),
        col: node.fetch(:position).fetch(:column),
        name: node.fetch(:name),
        type: translate(node.fetch(:var_type)),
      )
    when :named_type
      TypeName.new(name: node.fetch(:name))
    when :list_type
      ListType.new(of_type: translate(node.fetch(:type)))
    when :non_null_type
      NonNullType.new(of_type: translate(node.fetch(:type)))
    when :argument
      Argument.new(
        name: node.fetch(:name),
        value: translate(node.fetch(:value))
      )
    when :variable
      VariableIdentifier.new(name: node.fetch(:name))
    when :int, :float, :string, :boolean
      node.fetch(:value)
    when :null
      NullValue.new(name: "null")
    when :enum
      Enum.new(name: node.fetch(:value))
    when :list
      node.fetch(:value).map{|x| translate(x)}
    when :object
      arguments = node.fetch(:value).map{|x| translate(x)}
      InputObject.new(arguments:)
    end
  end  
end
