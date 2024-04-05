var searchIndex = new Map(JSON.parse('[\
["anchor_chain",{"doc":"Anchor Chain","t":"CCCCCCCCFFFNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNFNNNNNNNOONNNNNNFPGPFNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNCCFFFFNNNNNNNNNNNNNNOONNNNNNNNNNNNNNNNNNNNNNNNONNNOONNNNNNNNNNNNNNNNNNNNNNNPPPFFGNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNRKRFNNNNNNNNMNNNNNNFNNNNONNNONHNNNNNFNNNNNNNNNNNNNFNNNNNNNNNNNNN","n":["chain","link","message","models","node","parallel_node","prompt","trace_node","Chain","ChainBuilder","LinkedChainBuilder","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","build","default","fmt","from","from","from","into","into","into","into_shared","into_shared","into_shared","link","link","new","new","new_with_trace","process","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","vzip","vzip","vzip","Link","borrow","borrow_mut","fmt","from","into","into_shared","new","next","node","process","try_from","try_into","try_into","type_id","vzip","AssisnantChatMessage","Assistant","ChatMessage","User","UserChatMessage","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","from","from","from","into","into","into","into_shared","into_shared","into_shared","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","vzip","vzip","vzip","claude_3","openai","Claude3Bedrock","ClaudeImageSource","ClaudeMessage","ClaudeMessageContent","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","clone","clone","clone","clone_into","clone_into","clone_into","content","content_type","deserialize","deserialize","deserialize","fmt","fmt","fmt","fmt","from","from","from","from","from","from","from","into","into","into","into","into_shared","into_shared","into_shared","into_shared","new","process","role","serialize","serialize","serialize","source","text","to_owned","to_owned","to_owned","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","vzip","vzip","vzip","vzip","GPT3_5Turbo","GPT3_5TurboInstruct","GPT4Turbo","OpenAIChatModel","OpenAIInstructModel","OpenAIModel","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","fmt","fmt","fmt","from","from","from","into","into","into","into_shared","into_shared","into_shared","new_gpt3_5_turbo","new_gpt3_5_turbo_instruct","new_gpt4_turbo","new_with_key","new_with_key","process","process","process","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","vzip","vzip","vzip","Input","Node","Output","PassthroughNode","borrow","borrow_mut","default","fmt","from","into","into_shared","new","process","process","try_from","try_into","try_into","type_id","vzip","ParallelNode","borrow","borrow_mut","fmt","from","function","into","into_shared","new","nodes","process","to_boxed_future","try_from","try_into","try_into","type_id","vzip","Prompt","borrow","borrow_mut","fmt","from","into","into_shared","new","process","try_from","try_into","try_into","type_id","vzip","TraceNode","borrow","borrow_mut","fmt","from","into","into_shared","new","process","try_from","try_into","try_into","type_id","vzip"],"q":[[0,"anchor_chain"],[8,"anchor_chain::chain"],[50,"anchor_chain::link"],[66,"anchor_chain::message"],[101,"anchor_chain::models"],[103,"anchor_chain::models::claude_3"],[176,"anchor_chain::models::openai"],[223,"anchor_chain::node"],[242,"anchor_chain::parallel_node"],[259,"anchor_chain::prompt"],[273,"anchor_chain::trace_node"],[287,"core::marker"],[288,"core::marker"],[289,"core::fmt"],[290,"core::result"],[291,"core::future::future"],[292,"alloc::boxed"],[293,"core::pin"],[294,"core::any"],[295,"serde::de"],[296,"alloc::string"],[297,"serde::ser"],[298,"async_openai::types::chat"],[299,"core::convert"],[300,"async_openai::types::chat"],[301,"alloc::vec"],[302,"core::ops::function"]],"d":["Provides structures for creating and executing chains.","A link in a processing chain that connects one <code>Node</code> to …","Contains generic message types that can be used with …","Contains nodes that are designed to work with various LLM …","Module providing foundational structures for building …","Provides a structure for processing input through multiple …","Module for handling dynamic prompts in processing chains.","","Represents a chain of nodes that can asynchronously …","A builder for constructing a <code>Chain</code> of nodes.","A builder for constructing a <code>Chain</code> of nodes using Link.","","","","","","","Finalizes the construction of the chain, returning a <code>Chain</code> …","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","Adds a new node to the chain, linking it to the previous …","Adds a new node to the chain, linking it to the previous …","Creates a new <code>ChainBuilder</code> instance.","Creates a new <code>Chain</code> from the provided initial link.","Creates a new <code>ChainBuilder</code> instance with tracing enabled.","Asynchronously processes the provided input through the …","","","","","","","","","","","","","","","","A link in a processing chain that connects one <code>Node</code> to …","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","Creates a new <code>Link</code> connecting the specified nodes.","The next node or link in the chain.","The first node in the chain.","Processes the given input through the chain of nodes.","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","","","Module for interfacing with Claude 3 via AWS Bedrock.","Module for integrating OpenAI models.","A processor for integrating Claude 3 LLM processing within …","Represents a source of an image to be processed by Claude …","Represents a message to be sent to Claude 3, comprising …","Defines the content of a message for Claude 3, …","","","","","","","","","","","","","","","A vector of content items within the message.","The content type, e.g., “text”.","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","Constructs a new <code>Claude3Bedrock</code> processor with the …","Processes the input through the Claude 3 model, returning …","The role of the message, e.g., “user”.","","","","An image source, if applicable.","The actual text content, if applicable.","","","","","","","","","","","","","","","","","","","","","","","","GPT-3.5 Turbo model","GPT-3.5 Turbo Instruct model","GPT-4 Turbo model","Represents a processor for sending and processing requests …","Processor for making requests to OpenAI Instruct models.","OpenAI model types supported by the <code>OpenAI</code> node","","","","","","","Formats the <code>OpenAI</code> processor for debugging.","Formats the <code>OpenAI</code> processor for debugging.","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","Constructs a GPT3.5 Turbo model with the specified system …","Constructs a GPT3.5 Turbo Instruct model with the …","Constructs a GPT4 Turbo model with the specified system …","Constructs a new <code>OpenAI</code> processor with a specified API key.","Constructs a new <code>OpenAI</code> processor with a specified API key.","Sends the input to the OpenAI API and processes the …","Sends the input to the OpenAI API and processes the …","Sends the input to the OpenAI model and processes the …","","","","","","","","","","","","","","","","The input type for the node.","Represents an node that can process an input to produce an …","The output type for the node.","A no-op node that passes input through unchanged.","","","Creates a default <code>PassthroughNode</code>.","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","Creates a new <code>PassthroughNode</code>.","Asynchronously processes the given input, returning the …","Passes the input through unchanged.","","","","","","A node that processes input through multiple nodes in …","","","","Returns the argument unchanged.","The function to process the output of the nodes.","Calls <code>U::from(self)</code>.","","Creates a new <code>ParallelNode</code> with the provided nodes and …","The nodes that will process the input in parallel.","Processes the given input through nodes in parallel.","Converts a function into a boxed future that can be used …","","","","","","A processor for handling text prompts within a processing …","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","Creates a new <code>Prompt</code> processor with the specified text.","Processes the input by simply logging the prompt text and …","","","","","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","","","","","",""],"i":[0,0,0,0,0,0,0,0,0,0,0,8,1,2,8,1,2,1,8,2,8,1,2,8,1,2,8,1,2,8,1,8,2,8,2,8,1,2,8,8,1,1,2,2,8,1,2,8,1,2,0,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,0,27,0,27,0,27,26,28,27,26,28,27,26,28,27,26,28,27,26,28,27,26,28,27,27,26,26,28,28,27,26,28,27,26,28,0,0,0,0,0,0,25,20,21,22,25,20,21,22,20,21,22,20,21,22,22,21,20,21,22,25,20,21,22,25,20,21,22,22,22,22,25,20,21,22,25,20,21,22,25,25,22,20,21,22,21,21,20,21,22,25,20,21,22,25,25,20,20,21,21,22,22,25,20,21,22,25,20,21,22,35,35,35,0,0,0,31,32,35,31,32,35,31,32,35,31,32,35,31,32,35,31,32,35,35,35,35,31,32,31,32,35,31,32,35,31,31,32,32,35,35,31,32,35,31,32,35,4,0,4,0,37,37,37,37,37,37,37,37,4,37,37,37,37,37,37,0,38,38,38,38,38,38,38,38,38,38,0,38,38,38,38,38,0,42,42,42,42,42,42,42,42,42,42,42,42,42,0,44,44,44,44,44,44,44,44,44,44,44,44,44],"f":"```````````{ce{}{}}00000{{{b{ce}}}{{d{ce}}}f{h{h{}{{j{c}}}}fln}}{{}A`}{{{d{ceg}}Ab}Adnnn}{cc{}}00444444{{A`e}{{b{ce}}}f{{h{}{{j{c}}}}fln}}{{{b{ce}}g}{{b{c{Af{eg}}}}}f{{h{}{{j{c}}}}fln}{{h{}{{j{}}}}fln}}4{{gAh}{{d{ceg}}}nn{{h{}{{j{c}}{Aj{e}}}}fln}}5{{{d{ceg}}c}{{Al{e}}}nn{{h{}{{j{c}}{Aj{e}}}}fln}}{c{{An{e}}}{}{}}00{c{{Bd{{Bb{B`}}}}}{}}10101{cBf{}}00;;;`;;{{{Af{ce}}Ab}Ad{nn}{nn}}8<<{{ce}{{Af{ce}}}nn}``{{{Af{ce}}g}{{Bd{{Bb{B`}}}}}{hfln}{{h{}{{j{}}}}fln}{}}5543>`````>>>>>>:::>>>>>>555454545333>>>``````>>>>>>>>{BhBh}{BjBj}{BlBl}{{ce}Bn{}{}}00``{c{{An{Bh}}}C`}{c{{An{Bj}}}C`}{c{{An{Bl}}}C`}{{CbAb}Ad}{{BhAb}Ad}{{BjAb}Ad}{{BlAb}Ad}{cc{}}000{CdBl}{CfBl}{ChBl}{ce{}{}}0000000{CjCb}{{Cbc}{{Bd{{Bb{B`}}}}}{}}`{{Bhc}AnCl}{{Bjc}AnCl}{{Blc}AnCl}``555{c{{An{e}}}{}{}}0000{c{{Bd{{Bb{B`}}}}}{}}011010{cBf{}}0008888``````888888{{{Cn{c}}Ab}Ad{}}{{{D`{c}}Ab}Ad{{Dd{Db}}}}{{{Df{c}}Ab}Ad{{Dd{Db}}n}}???;;;;;;{Cj{{Df{c}}}{{Dd{Db}}}}{{}{{Df{c}}}{{Dd{Db}}}}1{{CjCjCj}{{Cn{c}}}{}}{{CjCj}{{D`{c}}}{{Dd{Db}}}}{{{Cn{c}}e}{{Bd{{Bb{B`}}}}}{{Dd{Dh}}fl}{}}{{{D`{c}}e}{{Bd{{Bb{B`}}}}}{{Dd{Db}}fl}{}}{{{Df{c}}e}{{Bd{{Bb{B`}}}}}{fln{Dd{Db}}{Dd{Dh}}}{}}<<<;<;<;<:::{ce{}{}}00````00{{}{{Dj{c}}}{}}{{{Dj{c}}Ab}Adn}{cc{}}332{{{h{}{{j{c}}{Aj{e}}}}c}{{Bd{{Bb{B`}}}}}{}{}}{{{Dj{c}}e}{{Bd{{Bb{B`}}}}}{fln}{}}{c{{An{e}}}{}{}}{c{{Bd{{Bb{B`}}}}}{}}1{cBf{}}8`88{{{Dl{ce}}Ab}Ad{nDnfl}{nfl}}6`99{{{E`{{Bb{h}}}}{Bb{Eb}}}{{Dl{ce}}}{Dnfl}{fl}}`{{{Dl{ce}}g}{{Bd{{Bb{B`}}}}}{Dnfln}{fln}{}}{e{{Bb{Eb}}}f{{Eb{c}{{Aj{{Al{Cj}}}}}}flDn}}6654<`<<{{EdAb}Ad}:=={EfEd}{{Edc}{{Bd{{Bb{B`}}}}}{}}9987?`??{{{Eh{c}}Ab}Ad{nh}}={ce{}{}}0{c{{Eh{c}}}h}{{{Eh{c}}e}{{Bd{{Bb{B`}}}}}{hfl}{}}==<;2","c":[],"p":[[5,"LinkedChainBuilder",8],[5,"Chain",8],[10,"Send",287],[10,"Node",223],[17,"Input"],[10,"Sync",287],[10,"Debug",288],[5,"ChainBuilder",8],[5,"Formatter",288],[8,"Result",288],[5,"Link",50],[1,"bool"],[17,"Output"],[8,"Result",289],[6,"Result",290],[10,"Future",291],[5,"Box",292],[5,"Pin",293],[5,"TypeId",294],[5,"ClaudeImageSource",103],[5,"ClaudeMessageContent",103],[5,"ClaudeMessage",103],[1,"unit"],[10,"Deserializer",295],[5,"Claude3Bedrock",103],[5,"UserChatMessage",66],[6,"ChatMessage",66],[5,"AssisnantChatMessage",66],[5,"String",296],[10,"Serializer",297],[5,"OpenAIChatModel",176],[5,"OpenAIInstructModel",176],[6,"Prompt",298],[10,"Into",299],[6,"OpenAIModel",176],[6,"ChatCompletionRequestUserMessageContent",298],[5,"PassthroughNode",223],[5,"ParallelNode",242],[10,"Clone",300],[5,"Vec",301],[10,"Fn",302],[5,"Prompt",259],[1,"str"],[5,"TraceNode",273]],"b":[[134,"impl-From%3CUserChatMessage%3E-for-ClaudeMessage"],[135,"impl-From%3CChatMessage%3E-for-ClaudeMessage"],[136,"impl-From%3CAssisnantChatMessage%3E-for-ClaudeMessage"]]}]\
]'));
if (typeof exports !== 'undefined') exports.searchIndex = searchIndex;
else if (window.initSearch) window.initSearch(searchIndex);
