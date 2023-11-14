# frozen_string_literal: true

RSpec.describe RustGraphqlParser do
  it "has a version number" do
    expect(RustGraphqlParser::VERSION).not_to be nil
  end

  specify do
    # pp RustGraphqlParser.parse("fragment MyFragment on Foo { value }")
    expect(RustGraphqlParser.parse("fragment MyFragment on Foo { value }")).to eq(
      {:node_type=>:document,
      :definitions=>
       [{:node_type=>:fragment_definition,
         :name=>"MyFragment",
         :position=>{:line=>1, :column=>1},
         :selection_set=>
          {:node_type=>:selection_set,
           :span=>[{:line=>1, :column=>28}, {:line=>1, :column=>36}],
           :items=>[{:node_type=>:field, :name=>"value", :position=>{:line=>1, :column=>30}, :selection_set=>{:node_type=>:selection_set, :span=>[{:line=>1, :column=>30}, {:line=>1, :column=>30}], :items=>[]}, :arguments=>[], :directives=>[]}]},
         :type_condition=>{:on=>"Foo"}}]}
    )
  end
end
