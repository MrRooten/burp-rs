# burp-rs

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
