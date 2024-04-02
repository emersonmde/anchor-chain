var searchIndex = new Map(JSON.parse('[\
["anchor_chain",{"doc":"Anchor Chain: A Rust Framework for Large Language Models …","t":"CCCCCCCFFNNNNNNNNNNNNNNNNNNNNNNNNNNFNNNNNNOONNNNNNFPGPFNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNCCFFFFNNNNNNNNNNNNNNOONNNNNNNNNNNNNNNNNNNNNNNONNNOONNNNNNNNNNNNNNNNNNNNNNNFNNNNNNNNNNNNNRKRFNNNNNNNMNNNNNNFNNNONNNONNNNNNFNNNNNNNNNNNN","n":["chain","link","message","models","node","parallel_node","prompt","Chain","ChainBuilder","borrow","borrow","borrow_mut","borrow_mut","build","from","from","into","into","into_shared","into_shared","link","new","new","process","try_from","try_from","try_into","try_into","try_into","try_into","type_id","type_id","vzip","vzip","with_trace","Link","borrow","borrow_mut","from","into","into_shared","new","next","node","process","try_from","try_into","try_into","type_id","vzip","AssisnantChatMessage","Assistant","ChatMessage","User","UserChatMessage","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","from","from","from","into","into","into","into_shared","into_shared","into_shared","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","vzip","vzip","vzip","claude_3","gpt_3_5_turbo","Claude3Bedrock","ClaudeImageSource","ClaudeMessage","ClaudeMessageContent","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","clone","clone","clone","clone_into","clone_into","clone_into","content","content_type","deserialize","deserialize","deserialize","fmt","fmt","fmt","from","from","from","from","from","from","from","into","into","into","into","into_shared","into_shared","into_shared","into_shared","new","process","role","serialize","serialize","serialize","source","text","to_owned","to_owned","to_owned","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","vzip","vzip","vzip","vzip","Gpt3_5Turbo","borrow","borrow_mut","from","into","into_shared","new","new_with_key","process","try_from","try_into","try_into","type_id","vzip","Input","Node","Output","PassthroughNode","borrow","borrow_mut","default","from","into","into_shared","new","process","process","try_from","try_into","try_into","type_id","vzip","ParallelNode","borrow","borrow_mut","from","function","into","into_shared","new","nodes","process","try_from","try_into","try_into","type_id","vzip","Prompt","borrow","borrow_mut","from","into","into_shared","new","process","try_from","try_into","try_into","type_id","vzip"],"q":[[0,"anchor_chain"],[7,"anchor_chain::chain"],[35,"anchor_chain::link"],[50,"anchor_chain::message"],[85,"anchor_chain::models"],[87,"anchor_chain::models::claude_3"],[159,"anchor_chain::models::gpt_3_5_turbo"],[173,"anchor_chain::node"],[191,"anchor_chain::parallel_node"],[206,"anchor_chain::prompt"],[219,"core::marker"],[220,"core::marker"],[221,"core::result"],[222,"core::future::future"],[223,"alloc::boxed"],[224,"core::pin"],[225,"core::any"],[226,"serde::de"],[227,"core::fmt"],[228,"core::fmt"],[229,"serde::ser"],[230,"alloc::vec"],[231,"core::ops::function"],[232,"core::clone"]],"d":["Provides structures for creating and executing chains.","","","","Module providing foundational structures for building …","","Module for handling dynamic prompts in processing chains.","Represents a chain of nodes that can asynchronously …","A builder for constructing a <code>Chain</code> of nodes.","","","","","Finalizes the construction of the chain, returning a <code>Chain</code> …","Returns the argument unchanged.","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","Adds a new node to the chain, linking it to the previous …","Creates a new <code>Chain</code> from the provided initial link.","Initializes a new <code>ChainBuilder</code> with the provided starting …","Asynchronously processes the provided input through the …","","","","","","","","","","","","A link in a processing chain that connects one <code>Node</code> to …","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","Creates a new <code>Link</code> connecting the specified nodes.","The next node or link in the chain.","The first node in the chain.","Processes the given input through the chain of nodes.","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","","","Module for interfacing with Claude 3 via AWS Bedrock.","Module for integrating GPT-3.5 Turbo model interactions.","A processor for integrating Claude 3 LLM processing within …","Represents a source of an image to be processed by Claude …","Represents a message to be sent to Claude 3, comprising …","Defines the content of a message for Claude 3, …","","","","","","","","","","","","","","","A vector of content items within the message.","The content type, e.g., “text”.","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","Returns the argument unchanged.","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","Constructs a new <code>Claude3Bedrock</code> processor with the …","Processes the input through the Claude 3 model, returning …","The role of the message, e.g., “user”.","","","","An image source, if applicable.","The actual text content, if applicable.","","","","","","","","","","","","","","","","","","","","","","","","Represents a processor for sending and processing requests …","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","Constructs a new <code>Gpt3_5Turbo</code> processor with the default …","Constructs a new <code>Gpt3_5Turbo</code> processor with a specified …","Asynchronously sends the input to the GPT-3.5 Turbo model …","","","","","","The input type for the node.","Represents an node that can process an input to produce an …","The output type for the node.","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","Asynchronously processes the given input, returning the …","","","","","","","","","","Returns the argument unchanged.","The function to process the output of the node.","Calls <code>U::from(self)</code>.","","Creates a new <code>ParallelNode</code> with the provided nodes and …","The node to process the input.","Processes the given input through nodes in parallel.","","","","","","A processor for handling text prompts within a processing …","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","Creates a new <code>Prompt</code> processor with the specified text.","Processes the input by simply logging the prompt text and …","","","","",""],"i":[0,0,0,0,0,0,0,0,0,2,1,2,1,1,2,1,2,1,2,1,1,2,1,2,2,1,2,2,1,1,2,1,2,1,1,0,7,7,7,7,7,7,7,7,7,7,7,7,7,7,0,23,0,23,0,23,24,25,23,24,25,23,24,25,23,24,25,23,24,25,23,24,25,23,23,24,24,25,25,23,24,25,23,24,25,0,0,0,0,0,0,27,16,17,18,27,16,17,18,16,17,18,16,17,18,18,17,16,17,18,16,17,18,27,16,17,18,18,18,18,27,16,17,18,27,16,17,18,27,27,18,16,17,18,17,17,16,17,18,27,16,17,18,27,27,16,16,17,17,18,18,27,16,17,18,27,16,17,18,0,29,29,29,29,29,29,29,29,29,29,29,29,29,4,0,4,0,30,30,30,30,30,30,30,4,30,30,30,30,30,30,0,32,32,32,32,32,32,32,32,32,32,32,32,32,32,0,36,36,36,36,36,36,36,36,36,36,36,36],"f":"`````````{ce{}{}}000{{{b{ce}}}{{d{ce}}}f{h{h{}{{j{c}}}}fl}}{cc{}}02222{{{b{ce}}g}{{b{c{n{eg}}}}}f{{h{}{{j{c}}}}fl}{{h{}{{j{}}}}fl}}{{gA`}{{d{ceg}}}{}{}{{h{}{{j{c}}{Ab{e}}}}fl}}{e{{b{ce}}}f{{h{}{{j{c}}}}fl}}{{{d{ceg}}c}{{Ad{e}}}{}{}{{h{}{{j{c}}{Ab{e}}}}fl}}{c{{Af{e}}}{}{}}00{c{{Al{{Aj{Ah}}}}}{}}10{cAn{}}099{{{b{ce}}A`}{{b{ce}}}f{{h{}{{j{c}}}}fl}}`::8::{{ce}{{n{ce}}}{}{}}``{{{n{ce}}g}{{Al{{Aj{Ah}}}}}{hfl}{{h{}{{j{}}}}fl}{}}5453<`````<<<<<<:::<<<<<<555455445333<<<``````<<<<<<<<{B`B`}{BbBb}{BdBd}{{ce}Bf{}{}}00``{c{{Af{B`}}}Bh}{c{{Af{Bb}}}Bh}{c{{Af{Bd}}}Bh}{{B`Bj}Bl}{{BbBj}Bl}{{BdBj}Bl}{cc{}}00{BnBd}1{C`Bd}{CbBd}{ce{}{}}0000000{CdCf}{{Cfc}{{Al{{Aj{Ah}}}}}{}}`{{B`c}AfCh}{{Bbc}AfCh}{{Bdc}AfCh}``555{c{{Af{e}}}{}{}}000{c{{Al{{Aj{Ah}}}}}{}}1100101{cAn{}}0008888`88<88{CdCj}{{CdCd}Cj}{{Cjc}{{Al{{Aj{Ah}}}}}{}}5543;````;;{{}{{Cl{c}}}{}}{cc{}}==1{{{h{}{{j{c}}{Ab{e}}}}c}{{Al{{Aj{Ah}}}}}{}{}}{{{Cl{c}}e}{{Al{{Aj{Ah}}}}}{fl}{}}9897?`??2`??{{{Cn{{Aj{h}}}}e}{{D`{gce}}}{fl}{{Db{{Cn{c}}}{{Ab{{Ad{c}}}}}}}{Ddfl}}`{{{D`{ceg}}i}{{Al{{Aj{Ah}}}}}{Ddfl}{fl}{{Db{{Cn{e}}}{{Ab{{Ad{e}}}}}}fl}{}};;:9{ce{}{}}`00500{DfDh}{{Dhc}{{Al{{Aj{Ah}}}}}{}}>=><2","c":[],"p":[[5,"ChainBuilder",7],[5,"Chain",7],[10,"Send",219],[10,"Node",173],[17,"Input"],[10,"Sync",219],[5,"Link",35],[1,"bool"],[17,"Output"],[8,"Result",220],[6,"Result",221],[10,"Future",222],[5,"Box",223],[5,"Pin",224],[5,"TypeId",225],[5,"ClaudeImageSource",87],[5,"ClaudeMessageContent",87],[5,"ClaudeMessage",87],[1,"unit"],[10,"Deserializer",226],[5,"Formatter",227],[8,"Result",227],[6,"ChatMessage",50],[5,"UserChatMessage",50],[5,"AssisnantChatMessage",50],[5,"String",228],[5,"Claude3Bedrock",87],[10,"Serializer",229],[5,"Gpt3_5Turbo",159],[5,"PassthroughNode",173],[5,"Vec",230],[5,"ParallelNode",191],[10,"Fn",231],[10,"Clone",232],[1,"str"],[5,"Prompt",206]],"b":[[116,"impl-From%3CChatMessage%3E-for-ClaudeMessage"],[118,"impl-From%3CUserChatMessage%3E-for-ClaudeMessage"],[119,"impl-From%3CAssisnantChatMessage%3E-for-ClaudeMessage"]]}]\
]'));
if (typeof exports !== 'undefined') exports.searchIndex = searchIndex;
else if (window.initSearch) window.initSearch(searchIndex);
