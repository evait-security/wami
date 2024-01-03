
<img src="assets/wami_logo.png" width="200px">

# WAMI - What am I
## Description:
WAMI is a user-friendly command line tool designed in Rust language, powered by Cargo, to assist individuals who struggle with remembering the names of the various programs they utilize. This open-source program aims to simplify the process of finding the most suitable programs for specific tasks.

## Features:
1. Program Recommendation: Wami analyzes the requirements of a given task and suggests the optimal programs to accomplish it efficiently.

2. User-Friendly Interface: The intuitive interface of Wami ensures a seamless experience for users of all skill levels.

3. Customization: Users can personalize their preferences and prioritize certain programs based on their individual needs.

## Templates
WAMI will use the official [wami-templates](https://github.com/evait-security/wami-templates) repository as a datasource. Please refer to this repository if you want to add a new tool suggestion.

## How it Works:
WAMI utilizes advanced algorithms written in Rust to evaluate the characteristics and functionalities of different programs. By matching the task requirements with program capabilities, it generates recommendations that maximize productivity.

## Contributions:
Contributions to Wami are highly encouraged. Developers can add new programs, improve existing algorithms, and enhance the user interface. By collaborating on this project, we can create a comprehensive program repository for the benefit of all users.

## Installation:
To install Wami, follow these steps:
1. [Download the binary](https://github.com/evait-security/wami/releases/download/linux_x64/wami)
3. ``` chmod +x wami ```
4. ``` ./wami ```

```code
wget https://github.com/evait-security/wami/releases/download/linux_x64/wami
chmod +x wami
./wami
```

## Workflow
Use ``` wami --help ``` for a list of options

### Example using the lake
``` ./wami -M 2 -S desc -s dns lookup hacking ```

### Result using the lake
```
2: ADM DNS spoofing tools
    https://packetstormsecurity.com/files/10080/ADMid-pkg.tgz.html
    https://packetstormsecurity.com/files/download/10080/ADMid-pkg.tgz
1: host - DNS lookup utility
    https://manned.org/host
```

### Example using github
``` ./wami --github -M 2 -S desc -s dns lookup hacking ```

### Result using github
```
2. Hacking_tool - Hi Friends of the hacking-tool script includes 9 tools in the data collection category such as: Whois | Port Scanner | Ping GeoIP | DNS Lookup Admin Finder | Headers | Reverse IP Cloudflare Bypasser This tool can be used
  - url: https://github.com/Cyber-Security-856/Hacking_tool
  - Stars: 9
  - Topics
  - Last update at: 2023-12-11T05:02:03Z
  - Score of finding: 1
1. go-mockdns - Boilerplate for testing of code involving DNS lookups, including unholy hacks to redirect net.Lookup* calls.
  - url: https://github.com/foxcpp/go-mockdns
  - Stars: 37
  - Topics
    - dns
    - golang
    - testing
  - Last update at: 2023-10-08T13:38:52Z
  - Score of finding: 1
```

### Explaining the parameters
* ``` -M 2 ``` Set the maximum of listed programs default (MAX = 10)
* ``` -S desc ``` This will determine the sorting direction asc or desc
* ``` -s dns lookup hacking ``` The search all functionality will search throw all fields with the same search criteria.

## Feedback and Support:
We value your feedback and appreciate any bug reports or suggestions. Please open an issue on our GitHub repository to provide feedback or seek support.

## License:
WAMI is released under the MIT License. Feel free to modify, distribute, and use the program for personal or commercial purposes.

Let's simplify program management and boost productivity with Wami! Together, we can conquer the challenge of remembering program names using the power of Rust and Cargo.
