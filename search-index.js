var searchIndex = new Map(JSON.parse('[\
["anchor_chain",{"doc":"Anchor Chain: A Rust Framework for Large Language Models …","t":"CCCCCCCFFFNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNFNNNNNNNOONNNNNNFPGPFNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNCCFFFFNNNNNNNNNNNNNNOONNNNNNNNNNNNNNNNNNNNNNNNONNNOONNNNNNNNNNNNNNNNNNNNNNNPPPFFGNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNRKRFNNNNNNNNMNNNNNNFNNNNONNNONHNNNNNFNNNNNNNNNNNNN","n":["chain","link","message","models","node","parallel_node","prompt","Chain","ChainBuilder","LinkedChainBuilder","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","build","default","fmt","from","from","from","into","into","into","into_shared","into_shared","into_shared","link","link","new","new","new_with_trace","process","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","vzip","vzip","vzip","Link","borrow","borrow_mut","fmt","from","into","into_shared","new","next","node","process","try_from","try_into","try_into","type_id","vzip","AssisnantChatMessage","Assistant","ChatMessage","User","UserChatMessage","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","from","from","from","into","into","into","into_shared","into_shared","into_shared","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","vzip","vzip","vzip","claude_3","openai","Claude3Bedrock","ClaudeImageSource","ClaudeMessage","ClaudeMessageContent","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","clone","clone","clone","clone_into","clone_into","clone_into","content","content_type","deserialize","deserialize","deserialize","fmt","fmt","fmt","fmt","from","from","from","from","from","from","from","into","into","into","into","into_shared","into_shared","into_shared","into_shared","new","process","role","serialize","serialize","serialize","source","text","to_owned","to_owned","to_owned","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","vzip","vzip","vzip","vzip","GPT3_5Turbo","GPT3_5TurboInstruct","GPT4Turbo","OpenAIChatModel","OpenAIInstructModel","OpenAIModel","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","fmt","fmt","fmt","from","from","from","into","into","into","into_shared","into_shared","into_shared","new_gpt3_5_turbo","new_gpt3_5_turbo_instruct","new_gpt4_turbo","new_with_key","new_with_key","process","process","process","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","vzip","vzip","vzip","Input","Node","Output","PassthroughNode","borrow","borrow_mut","default","fmt","from","into","into_shared","new","process","process","try_from","try_into","try_into","type_id","vzip","ParallelNode","borrow","borrow_mut","fmt","from","function","into","into_shared","new","nodes","process","to_boxed_future","try_from","try_into","try_into","type_id","vzip","Prompt","borrow","borrow_mut","fmt","from","into","into_shared","new","process","try_from","try_into","try_into","type_id","vzip"],"q":[[0,"anchor_chain"],[7,"anchor_chain::chain"],[49,"anchor_chain::link"],[65,"anchor_chain::message"],[100,"anchor_chain::models"],[102,"anchor_chain::models::claude_3"],[175,"anchor_chain::models::openai"],[222,"anchor_chain::node"],[241,"anchor_chain::parallel_node"],[258,"anchor_chain::prompt"],[272,"core::marker"],[273,"core::marker"],[274,"core::fmt"],[275,"core::result"],[276,"core::future::future"],[277,"alloc::boxed"],[278,"core::pin"],[279,"core::any"],[280,"serde::de"],[281,"alloc::string"],[282,"serde::ser"],[283,"async_openai::types::chat"],[284,"core::convert"],[285,"core::convert"],[286,"alloc::vec"],[287,"core::ops::function"]],"d":["Provides structures for creating and executing chains.","","","","Module providing foundational structures for building …","Parallel Node","Module for handling dynamic prompts in processing chains.","Represents a chain of nodes that can asynchronously …","","A builder for constructing a <code>Chain</code> of nodes.","","","","","","","Finalizes the construction of the chain, returning a <code>Chain</code> …","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","Adds a new node to the chain, linking it to the previous …","","Creates a new <code>Chain</code> from the provided initial link.","","Asynchronously processes the provided input through the …","","","","","","","","","","","","","","","","A link in a processing chain that connects one <code>Node</code> to …","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","Creates a new <code>Link</code> connecting the specified nodes.","The next node or link in the chain.","The first node in the chain.","Processes the given input through the chain of nodes.","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","","","Module for interfacing with Claude 3 via AWS Bedrock.","Module for integrating OpenAI models.","A processor for integrating Claude 3 LLM processing within …","Represents a source of an image to be processed by Claude …","Represents a message to be sent to Claude 3, comprising …","Defines the content of a message for Claude 3, …","","","","","","","","","","","","","","","A vector of content items within the message.","The content type, e.g., “text”.","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","Constructs a new <code>Claude3Bedrock</code> processor with the …","Processes the input through the Claude 3 model, returning …","The role of the message, e.g., “user”.","","","","An image source, if applicable.","The actual text content, if applicable.","","","","","","","","","","","","","","","","","","","","","","","","","","","Represents a processor for sending and processing requests …","","OpenAI model types supported by the <code>OpenAI</code> node","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","Constructs a GPT3.5 Turbo model with the specified system …","Constructs a GPT3.5 Turbo Instruct model with the …","Constructs a GPT4 Turbo model with the specified system …","Constructs a new <code>OpenAI</code> processor with a specified API key.","Constructs a new <code>OpenAI</code> processor with a specified API key.","Sends the input to the OpenAI API and processes the …","Sends the input to the OpenAI API and processes the …","Sends the input to the OpenAI model and processes the …","","","","","","","","","","","","","","","","The input type for the node.","Represents an node that can process an input to produce an …","The output type for the node.","A no-op node that passes input through unchanged.","","","Creates a default <code>PassthroughNode</code>.","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","Creates a new <code>PassthroughNode</code>.","Asynchronously processes the given input, returning the …","Passes the input through unchanged.","","","","","","","","","","Returns the argument unchanged.","The function to process the output of the nodes.","Calls <code>U::from(self)</code>.","","Creates a new <code>ParallelNode</code> with the provided nodes and …","The nodes that will process the input in parallel.","Processes the given input through nodes in parallel.","","","","","","","A processor for handling text prompts within a processing …","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","Creates a new <code>Prompt</code> processor with the specified text.","Processes the input by simply logging the prompt text and …","","","","",""],"i":[0,0,0,0,0,0,0,0,0,0,8,1,2,8,1,2,1,8,2,8,1,2,8,1,2,8,1,2,8,1,8,2,8,2,8,1,2,8,8,1,1,2,2,8,1,2,8,1,2,0,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,0,26,0,26,0,26,27,28,26,27,28,26,27,28,26,27,28,26,27,28,26,27,28,26,26,27,27,28,28,26,27,28,26,27,28,0,0,0,0,0,0,25,20,21,22,25,20,21,22,20,21,22,20,21,22,22,21,20,21,22,25,20,21,22,25,20,21,22,22,22,22,25,20,21,22,25,20,21,22,25,25,22,20,21,22,21,21,20,21,22,25,20,21,22,25,25,20,20,21,21,22,22,25,20,21,22,25,20,21,22,35,35,35,0,0,0,31,32,35,31,32,35,31,32,35,31,32,35,31,32,35,31,32,35,35,35,35,31,32,31,32,35,31,32,35,31,31,32,32,35,35,31,32,35,31,32,35,4,0,4,0,38,38,38,38,38,38,38,38,4,38,38,38,38,38,38,0,39,39,39,39,39,39,39,39,39,39,0,39,39,39,39,39,0,43,43,43,43,43,43,43,43,43,43,43,43,43],"f":"``````````{ce{}{}}00000{{{b{ce}}}{{d{ce}}}f{h{h{}{{j{c}}}}fln}}{{}A`}{{{d{ceg}}Ab}Adnnn}{cc{}}00444444{{A`e}{{b{ce}}}f{{h{}{{j{c}}}}fln}}{{{b{ce}}g}{{b{c{Af{eg}}}}}f{{h{}{{j{c}}}}fln}{{h{}{{j{}}}}fln}}4{{gAh}{{d{ceg}}}nn{{h{}{{j{c}}{Aj{e}}}}fln}}5{{{d{ceg}}c}{{Al{e}}}nn{{h{}{{j{c}}{Aj{e}}}}fln}}{c{{An{e}}}{}{}}00{c{{Bd{{Bb{B`}}}}}{}}11001{cBf{}}00;;;`;;{{{Af{ce}}Ab}Ad{nn}{nn}}8<<{{ce}{{Af{ce}}}nn}``{{{Af{ce}}g}{{Bd{{Bb{B`}}}}}{hfln}{{h{}{{j{}}}}fln}{}}5543>`````>>>>>>:::>>>>>>555454545333>>>``````>>>>>>>>{BhBh}{BjBj}{BlBl}{{ce}Bn{}{}}00``{c{{An{Bh}}}C`}{c{{An{Bj}}}C`}{c{{An{Bl}}}C`}{{CbAb}Ad}{{BhAb}Ad}{{BjAb}Ad}{{BlAb}Ad}{cc{}}00{CdBl}{CfBl}{ChBl}3{ce{}{}}0000000{CjCb}{{Cbc}{{Bd{{Bb{B`}}}}}{}}`{{Bhc}AnCl}{{Bjc}AnCl}{{Blc}AnCl}``555{c{{An{e}}}{}{}}000{c{{Bd{{Bb{B`}}}}}{}}1010110{cBf{}}0008888``````888888{{{Cn{c}}Ab}Ad{}}{{{D`{c}}Ab}Ad{{Dd{Db}}}}{{{Df{c}}Ab}Ad{{Dh{Cj}}{Dd{Db}}n}}???;;;;;;{Cj{{Df{c}}}{{Dh{Cj}}{Dd{Db}}}}{{}{{Df{c}}}{{Dh{Cj}}{Dd{Db}}}}1{{CjCjCj}{{Cn{c}}}{}}{{CjCj}{{D`{c}}}{{Dd{Db}}}}{{{Cn{c}}e}{{Bd{{Bb{B`}}}}}{{Dh{Cj}}{Dd{Dj}}fl}{}}{{{D`{c}}e}{{Bd{{Bb{B`}}}}}{{Dh{Cj}}{Dd{Db}}fl}{}}{{{Df{c}}e}{{Bd{{Bb{B`}}}}}{{Dh{Cj}}fln{Dd{Db}}{Dd{Dj}}}{}}<<<<;;<;<:::{ce{}{}}00````00{{}{{Dl{c}}}{}}{{{Dl{c}}Ab}Adn}{cc{}}332{{{h{}{{j{c}}{Aj{e}}}}c}{{Bd{{Bb{B`}}}}}{}{}}{{{Dl{c}}e}{{Bd{{Bb{B`}}}}}{fln}{}}{c{{An{e}}}{}{}}{c{{Bd{{Bb{B`}}}}}{}}1{cBf{}}8`88{{{Dn{ce}}Ab}Ad{nE`fl}{nfl}}6`99{{{Eb{{Bb{h}}}}{Bb{Ed}}}{{Dn{ce}}}{E`fl}{fl}}`{{{Dn{ce}}g}{{Bd{{Bb{B`}}}}}{E`fln}{fln}{}}{e{{Bb{Ed}}}f{{Ed{c}{{Aj{{Al{Cj}}}}}}flE`}}6654<`<<{{EfAb}Ad}:=={EhEf}{{Efc}{{Bd{{Bb{B`}}}}}{}}9897?","c":[],"p":[[5,"LinkedChainBuilder",7],[5,"Chain",7],[10,"Send",272],[10,"Node",222],[17,"Input"],[10,"Sync",272],[10,"Debug",273],[5,"ChainBuilder",7],[5,"Formatter",273],[8,"Result",273],[5,"Link",49],[1,"bool"],[17,"Output"],[8,"Result",274],[6,"Result",275],[10,"Future",276],[5,"Box",277],[5,"Pin",278],[5,"TypeId",279],[5,"ClaudeImageSource",102],[5,"ClaudeMessageContent",102],[5,"ClaudeMessage",102],[1,"unit"],[10,"Deserializer",280],[5,"Claude3Bedrock",102],[6,"ChatMessage",65],[5,"UserChatMessage",65],[5,"AssisnantChatMessage",65],[5,"String",281],[10,"Serializer",282],[5,"OpenAIChatModel",175],[5,"OpenAIInstructModel",175],[6,"Prompt",283],[10,"Into",284],[6,"OpenAIModel",175],[10,"From",284],[6,"ChatCompletionRequestUserMessageContent",283],[5,"PassthroughNode",222],[5,"ParallelNode",241],[10,"Clone",285],[5,"Vec",286],[10,"Fn",287],[5,"Prompt",258],[1,"str"]],"b":[[132,"impl-From%3CChatMessage%3E-for-ClaudeMessage"],[133,"impl-From%3CUserChatMessage%3E-for-ClaudeMessage"],[134,"impl-From%3CAssisnantChatMessage%3E-for-ClaudeMessage"]]}]\
]'));
if (typeof exports !== 'undefined') exports.searchIndex = searchIndex;
else if (window.initSearch) window.initSearch(searchIndex);
