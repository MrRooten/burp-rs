# burp-rs //debug version, a lots of changes may not Forward Compatible and lots of unwrap not handle

burpsuite-like tools. Because the burp extension is hard to debug, so I write a burp-like tool

## How to use it
  ### Build and run
  ``git clone ${this}``  
  
  ``sudo apt install ruby``  

  ``cd burp-rs``  

  ``cargo build --release``  

  ``cp -r active/ target/release/``  
  
  ``cp -r libruby/ target/release/``  

  ``cd target/release``  

  ``./burp-rs``  
 ### Easy way
  If you got ruby install in your system, just ``git clone`` and ``cargo run``

![image](https://user-images.githubusercontent.com/25635931/207817203-c283640c-40df-45d1-a403-0b54e05abba9.png)  
  #### Extensions or Modules are writting in ruby or rust, ruby modules placed in ./active/ directory
 ### Commands
 #### list_history
 `burp-rs> list_history`
 
![image](https://user-images.githubusercontent.com/25635931/208865736-8d709660-2abd-4318-b7d3-b641590f0236.png)

`burp-rs> list_history cn.bing.com`  Some pages are not captured, because of the config.yaml not allow cature jpeg, you can modify the cature rule in config.yaml

![image](https://user-images.githubusercontent.com/25635931/208866101-7e56b950-9c3e-48e6-a041-1d121487bd05.png)


 #### active_scan
![image](https://user-images.githubusercontent.com/25635931/208836245-0d1166f6-2ded-4d4f-a2aa-c673490c4707.png)
 #### scan [#warning poc related commands have been rename to mod, like: loaded_pocs -> loaded_mods, running_pocs -> running_mods]
 ![image](https://user-images.githubusercontent.com/25635931/208837152-447be76f-e483-4382-9876-0aa3727506be.png)
 ![image](https://user-images.githubusercontent.com/25635931/208839723-7d3912cd-2e6f-4efc-a94c-c2d602c5b2d8.png)

 ![image](https://user-images.githubusercontent.com/25635931/208838313-0cc448fd-6b9f-4a9a-9bf8-c0a7f624fbb9.png)
 ![image](https://user-images.githubusercontent.com/25635931/208839540-eff42130-b333-4d04-90e9-7965e3d76a3f.png)
 ![image](https://user-images.githubusercontent.com/25635931/208838481-23b2a138-74af-4cb4-b6ad-308f55cb53ed.png)
 ![image](https://user-images.githubusercontent.com/25635931/208838573-83c01e57-4da6-40f4-b867-5eb6b834bb60.png)
 #### running_mods
 ![image](https://user-images.githubusercontent.com/25635931/208858444-e618ff4f-f5f1-486e-a66a-79be39f4dabf.png)
 #### log
 ![image](https://user-images.githubusercontent.com/25635931/208858632-aa70b3d1-0e44-4ddb-953c-9c6a064edd68.png)
 #### cat_resp/cat_req
![image](https://user-images.githubusercontent.com/25635931/208859193-be78a6c2-5879-479c-ba21-f8e1d93e1521.png)
![image](https://user-images.githubusercontent.com/25635931/208858998-545df78c-f1bb-40cd-adf1-cec300e3bab8.png)
![image](https://user-images.githubusercontent.com/25635931/208859289-5d959a19-34d0-4a58-847b-17e8d070290f.png)
 #### get_req (not format output yet)
 ![image](https://user-images.githubusercontent.com/25635931/208859600-bd9b2bef-f8df-418e-b7d2-ec9c8ccce4b9.png)
 #### debug_level
 ![image](https://user-images.githubusercontent.com/25635931/208859856-bd719b74-9fd8-47dc-b643-5f0d501d32b2.png)










