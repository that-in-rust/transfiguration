What we are a little sure about for Parseltongue at 202510160900 hrs


# Early Experience v1

1. User downloads binary from github OR compiles from source by cloning the repo
2. User goes to the Rust git repository he wants to work on and copies the binary to the root folder
3. User runs `./parseltongue` and is greeted by a UI which is very similar to Claude Code
4. Background task is triggered to tell the user
  - What is their system configuration - is it Apple Silicon with 16GB+ RAM OR NOT
    - If yes ask them if
      - they want to enter anthropic key OR
      - they want to trigger one-click setup for Ollama if it does NOT exist
        - if it exists ask them to enter the Ollama anthropic format key
          - export ANTHROPIC_BASE_URL=https://api.company.ai/api/anthropic OR http://localhost:934/v1 (inspired by platform 9 and 3 quarters)
          - export ANTHROPIC_AUTH_TOKEN=z000zzzZz000000z0z00zz00z00zz00z.Z0Zz0zZZzZzZzZzz
        - if it does not exist then they say Ollama_Yes to install it and then we trigger a set of commands to install Ollama with our pre-configured model qwen2.5-coder:7b
          - this will be often 0.5x slower than a usual anthropic paid api key and little lesser on the quality of reasoning but with context length 128k
          - we are simulating that in future the SOTA local LLMs will be faster than what they are today, so that users do not feel a lot of difference, and thus we will wait for the reality to arrive there
    - If no then ask them to enter anthropic key - comprehensive list of anthropic keys providers from z.ai to anthropic itself (Opinionated take of what order and comments we offer, so example we could z.ai is $ xx per unit token and ab performance against say anthropic which is better performance but higher cost and so on... eventually reminding them that if they had apple silicon of 16GB RAM then it would have been zero)
      - If they have anthropic key which works, then move to next step
      - If they do not have anthropic key then ask them exit the application or enter the anthropic key again
5. Now Ollama or anthropic is configured and validated else the user would have exited
6. Backgrounds tasks in progress
  - ISG_current is built and you can ask questions to the ISG_current using the chat with a default question being triggered
    - How many top level modules does my codebase have & what is the token size of my ISG_current?
    - This will prove that the LLM is working because without the LLM this would not have been possible
    - If ISG_current creation fails it auto-tries while telling the user
    - If user interrupts LLM suggests to trigger ISG_current creation again because that is the keystone of this tool

# Overall view v1

1. User arrives at the codebase via claude-code fork called Parseltongue
2. Interface Signature Graph is created in RAM as ISG_current
    - each node is an interface signature with a unique identifier
        - the identifier is filePath-fileName-InterfaceName where InterfaceName is defined such a way that it remains unique
    - any interface is a limited set of things but ALL of them should be at first level of distance from filename, which means interfaces which are inside other interfaces are not of relevant concern
    - the interface signature is enriched with meta-data from rust analyzer
        - might include HIR information
        - might include any information that helps understand what is the utility of this interface
        - might include any dependencies or etc. analytics related to its relationships with other interfaces
        - classification label of whether it is a test interface used for TDD or a normal interface
    - a persistent copy is created in a relevant graph database maybe CosData
    - a persistent copy is created in a JSON file
    - a visualization in HTML based on ISG which can help the user understand
        - control flow of codebase
        - overall structure of code
3. The codebase is copied into SQLlite database
    - Table Codebase has following columns
        - id
        - ISG_current_ind (0 or 1)
        - ISG_future_ind (0 or 1)
        - filePath-fileName-interfaceName as unique identifier
        - Current_Code
        - Future_Code (empty at initiation)
        - Future_Action (empty at initiation, but filled with whether to suit the PRD change we will edit or delete or create this new interface)
4. The user is asked to create a PRD
    - the LLM asks the user to refine the PRD in context of ISG_current
    - the PRD is created
5. The LLM is tasked with creating a new ISG_future which does not have a persistent copy based on
    - ISG_current + PRD
    - the LLM is asked if
        - Do we need to revise the PRD?
            - Yes
                - ISG_future is possible or NOT or should we ask for a PRD change based on whether the change is too big or too complicated - especially in terms of risks of different nature
            - No
                - if ISG_future is possible then lets have base value of ISG_current as a default value and then change it according to what you think is the correct logic
                    - what test-interfaces will be deleted, edited, created
                    - what non-test-interfaces will be deleted, edited, created
                - now reflect these ISG_future changes in the SQLlite database in ISG_current_ind, ISG_future_ind, Future_Code and Future_Action columns
                - now use the rubber duck debugging menthod to look at ISG_current + PRD + ISG_future + those rows in SQLlite database which have Future_Code and Future_Action columns as not null
                - if the LLM thinks that we need to refine the solutioning further, make changes to ISG_future and repeat the process
                - if the LLM thinks that we need to refine the PRD further then go back to previous step
                - if finally the LLM feels very confident of the changes, we reflect the changes in the SQLlite database in the codebase
                - now we run all the tests and compile the codebase
                - if the tests fail then we go back to previous step
                - if the tests pass then we show the visualization of changes in ISG to the user + results of compilation + tests + request behavorial confirmation
                - if user gives go ahead then we
                    - make a commit with list of changes
                    - recreate ISG_current from ISG_future; update the SQLlite database according to the current codebase from scratch

# Ideal Tech Stack or Structure

1. CozoDB for persistent storage of ISG_current and ISG_future
  - this will also have the current_code and future_code as relational --something something 
2. 