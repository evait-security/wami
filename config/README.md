## Folder Name: "config" for the project named "Wami"

### Description:

The "config" folder is an integral part of the "Wami" project and is intended to store program descriptions in the YAML format. Each file within this folder represents a program and follows a specific structure, including the following fields:

* id (string): A unique identifier assigned to the program.
* info (string): The parent category.
    * name (string): The name or title of the program.
    * tags (array of strings): An array of keywords or tags used to categorize and classify the program.
    * author (string): The name or identifier of the author who created the program.
    * description (string): A concise description or summary of the program's purpose, functionality, or features.
    * references (array of strings): An array of strings representing related resources or references associated with the program.

The example.yaml shows this structure.