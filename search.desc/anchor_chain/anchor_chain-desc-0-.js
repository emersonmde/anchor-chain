searchState.loadedDescShard("anchor_chain", 0, "Anchor Chain\nDefines errors types for Anchor Chain\nError when no response is returned from the LLM model.\nA link in a processing chain that connects one <code>Node</code> to …\nGeneric error calling a model.\nOccurs when failing to construct OpenAI prompts, messages …\nGeneric error that occurs when serializing or …\nError constructing or rendering Tera templates.\nProvides structures for creating and executing chains.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nContains nodes that are designed to work with various LLM …\nCreates a new <code>Link</code> connecting the specified nodes.\nThe next node or link in the chain.\nModule providing foundational structures for building …\nThe first node in the chain.\nVarious nodes that can be chained together to form an LLM …\nProvides a structure for processing input through multiple …\nProcesses the given input through the chain of nodes.\nThis module contains various nodes for working with Vector …\nRepresents a chain of nodes that can asynchronously …\nA builder for constructing a <code>Chain</code> of nodes.\nA builder for constructing a <code>Chain</code> of nodes using Link.\nFinalizes the construction of the chain, returning a <code>Chain</code> …\nFinalizes the construction of the chain, returning a <code>Chain</code> …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nAdds the first node to the chain.\nAdds a new node to the chain, linking it to the previous …\nAdds a new node to the chain, linking it to the previous …\nCreates a new <code>ChainBuilder</code> instance.\nCreates a new <code>Chain</code> from the provided initial link.\nAsynchronously processes the provided input through the …\nDefines the interface for an embedding model that can …\nModule for integrating OpenAI models.\nDefines the interface for an embedding model that can …\nEmbeds the given input text and returns the resulting …\nGPT-3.5 Turbo model\nGPT-3.5 Turbo Instruct model\nGPT-4 Turbo model\nRepresents a processor for sending and processing requests …\nNode for making requests to OpenAI embedding models.\nNode for making requests to OpenAI Instruct models.\nOpenAI model types supported by the <code>OpenAI</code> node\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nConstructs a GPT3.5 Turbo model with the specified system …\nConstructs a GPT3.5 Turbo Instruct model.\nConstructs a GPT4 Turbo model with the specified system …\nConstructs a new <code>OpenAI</code> node using the specified API key.\nConstructs a new <code>OpenAI</code> processor with a specified API key.\nSends the input to the OpenAI API and processes the …\nSends the input to the OpenAI API and processes the …\nSends the input to the OpenAI API and processes the …\nSends the prompt to the OpenAI model and processes the …\nThe input type for the node.\nA no-op node that passes input through unchanged.\nRepresents a node that can process an input to produce an …\nThe output type for the node.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCreates a new <code>NoOpNode</code>.\nAsynchronously processes the given input, returning the …\nReturns the input unchanged.\nA simple input logging node.\nModule for handling dynamic prompts in processing chains.\nA simple input logging node\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCreate a new Logger node with the given prefix.\nLog the input and pass it through unchanged.\nA processor for handling text prompts within a processing …\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCreates a new <code>Prompt</code> processor with the specified template.\nProcesses the input HashMap and returns the rendered …\nA node that processes input through multiple nodes in …\nReturns the argument unchanged.\nThe function to process the output of the nodes.\nCalls <code>U::from(self)</code>.\nCreates a new <code>ParallelNode</code> with the provided nodes and …\nThe nodes that will process the input in parallel.\nProcesses the given input through nodes in parallel.\nConverts a function into a <code>BoxFuture</code> that can be used in a …\nStructures for managing documents in vector databases.\nA struct representing a collection of documents.\nDocument structure for serializing and deserializing when …\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCreate a new document with the given text.\nCreate a new document with the given text and embedding.\nCreate a new document with the given id and text.")