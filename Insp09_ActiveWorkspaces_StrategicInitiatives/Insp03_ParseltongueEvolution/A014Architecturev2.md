Core Parseltongue Loop

- The LLM is tasked with creating a new ISG_future which does not have a persistent copy based on
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


