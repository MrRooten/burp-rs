# burp-rs //debug now, something change may not Forward Compatibility

burpsuite-like tools. Because the burp extension is hard to debug, so I write a burp-like tool

## How to use it
  ### Build and run
  ``git clone ${this}``  
  
  ``sudo apt install ruby``  

  ``cd burp-rs``  

  ``cargo build --release``  

  ``cp -r active/ target/release/``  

  ``cd target/release``  

  ``./burp-rs``  

![image](https://user-images.githubusercontent.com/25635931/207817203-c283640c-40df-45d1-a403-0b54e05abba9.png)  
  #### Extensions or Poc(Proof of Concept) is writting in ruby or rust, placed in ./active/ directory
 ### Commands
 #### list_history
![image](https://user-images.githubusercontent.com/25635931/207822587-318133ce-0239-4722-978b-e3cb9764b82a.png)
 #### active_scan
![image](https://user-images.githubusercontent.com/25635931/208836245-0d1166f6-2ded-4d4f-a2aa-c673490c4707.png)
 #### scan [#warning poc related commands have been rename to mod: loaded_pocs -> loaded_mods, running_pocs -> running_mods]
 ![image](https://user-images.githubusercontent.com/25635931/208837152-447be76f-e483-4382-9876-0aa3727506be.png)
 ![image](https://user-images.githubusercontent.com/25635931/208839723-7d3912cd-2e6f-4efc-a94c-c2d602c5b2d8.png)

 ![image](https://user-images.githubusercontent.com/25635931/208838313-0cc448fd-6b9f-4a9a-9bf8-c0a7f624fbb9.png)
 ![image](https://user-images.githubusercontent.com/25635931/208839540-eff42130-b333-4d04-90e9-7965e3d76a3f.png)
 ![image](https://user-images.githubusercontent.com/25635931/208838481-23b2a138-74af-4cb4-b6ad-308f55cb53ed.png)
 ![image](https://user-images.githubusercontent.com/25635931/208838573-83c01e57-4da6-40f4-b867-5eb6b834bb60.png)








