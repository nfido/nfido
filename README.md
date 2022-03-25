# NFido 论坛

致敬CFido 论坛


    上世纪九十年代初，互联网尚未普及之际，通过电话线路连接起来的 BBS 网络——CFido，曾是国内第一批网民
    的聚集地。他们在那里共同营造了一段中国互联网不可磨灭的，但却鲜为后来人了解的故事。
    
    
    许多互联网名人都曾在 CFido 活跃过，但在二十多年前互为网友的他们，尚不知此后的人生会发生怎样的变化。
    
    那时，丁磊不顾领导反对离开宁波电信局，来到广州准备闯一闯，润迅马化腾、金山雷军都正开发着各自的软件。
    多年以后马化腾回忆：「当年一起喝啤酒的时候，我们只是打工仔而已，都还不知道未来。」
    
    一群二三十岁的年轻人，通过一条条电话线连成的网络，聚在了一起。他们使用的网络 ID 多为真实姓氏或全名，
    字里行间透露着初代网友间纯真的网络友谊，在 CFido 上更是无所不谈。马化腾曾与网友交流歇后语「小母牛
    跳高——挺牛 B」；雷军因想要通过媒体推广 CFido 而引起讨论；求伯君用计算机术语写起过段子……

论坛基于 Rust 语言开发，使用PostgreSQL 数据库。

文档： https://nfido-doc.bsmi.info

网站： https://nfido.bsmi.info

文档源代码： https://github.com/nfido/nfido-doc


#  依赖

项目需要openssl 

```
macOS
$ brew install openssl@1.1

Arch Linux
$ sudo pacman -S pkg-config openssl

Debian and Ubuntu
$ sudo apt-get install pkg-config libssl-dev

Fedora
$ sudo dnf install pkg-config openssl-devel
```

# 许可证

简单介绍一下本项目的许可证,是AGPL, 如果你修改了本软件, 通网络提供公开服务,那么你必须开源代码(但是不包括你的什么账号密码这些私有信息)

我们希望你能从本项目获益,并且支持一下,回馈一下开源项目.

修改,新增的文件,源代码需要带上本许可证,并且附上注释

    Copyright (C) <2003-2022>  NFido Team

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as published
    by the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.

这一段文本以本项目的原始文本为准. 其中年份会随着年份来修改,今年是2022年,所以是2003-2022