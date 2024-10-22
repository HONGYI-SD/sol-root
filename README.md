# 业务流

## L1到L2跨链
1. 用户调用L1Bridge合约,并进行token转账.
2. L1Bridge合约调用L1Message将跨链消息进行封装,并抛出事件.
3. 执行节点监听Bridge指定事件,监听到后在L2生成系统交易.
4. 系统交易为对L2Message合约进行调用.
5. L2Message合约进行跨链消息解析,解析为对L2Bridge调用.
6. L2Bridge 执行finalize方法,进行最后token转移.

## L2到L1跨链
1. 用户调用L2Bridge合约,并进行token转移.
2. 等待执行节点提交状态根.
3. 用户调用L1Message合约(proveWithdrawalTransaction),上传Proof相关.
4. 从Root获取到根信息,进行比对,再进行Proof验证.
5. 用户调用L1Message合约(finalizeWithdrawalTransaction),时间检查后,调用L1Bridge 合约.
6. L1Bridge 执行finalize方法,进行最后token转移.

## 提交状态跟
1. 根据定期轮行查询是否需要提交到L1Root合约区块高度,如9000间隔(1小时).
2. 查询L2Message中SMT树中根,和L2世界中SMT中根.
3. 产生L1交易,讲根信息发送至L1Message中.

## 提交DA承诺
1. 定期轮询查看是否到需要提交到L1 DACommitment合约,如150间隔(1分钟).
2. 组织DA数据,生成KZG承诺.
3. 将DA 承诺发送至L1 DACommitment合约.

## 扫块
1. 通过调用API方法:getConfirmedSignaturesForAddress2,获取区块内跟合约地址相关交易.
2. 解析交易获取所需要的跨链信息.
3. 生成系统交易,产生到L2Message合约的调用.
4. 执行relayMessage,方法对跨链消息进行执行.
5. 调用L2Bridge合约,进行跨链执行,转账到用户token.


# L1 合约

## Config
1. 管理员账户地址
2. gas相关设置
3. 各类功能合约地址

## Bridge
1. deposit:存款token,供用户使用.
2. finalize:在用户进行L2 -> L1时,最终转移token的方法.

## Message
1. depositTransaction:L1到L2的跨链封装,供桥合约调用.
2. proveWithdrawalTransaction:L2到L1的跨链消息验证,供用户提交证明.
3. finalizeWithdrawalTransaction:L2到L1最终消息执行,供用户调用.

## DA commitment
1. L2 DA数据生成的承诺管理.

## Root
1. L2 全局世界状态根管理.
2. L2 Message中SMT树的状态跟管理.

## Challenge
1. 创建挑战.
2. 交互式的挑战相关.
3. 对Root和对DA commitment合约的调用.

# L2 合约

## Bridge
1. withdrawal:取款token,供用户使用.
2. finalize:在用户进行L1 -> L2时,最终转移token的方法.
## Message
1. initiateWithdrawal:L2到L1的跨链封装,供桥合约使用.
2. relayMessage:L1到L2的跨链执行,供执行节点生成系统交易调用.

# 问题
1. 组织DA数据方式.
2. L2区块时间,出块时间是否要短过L1.  






<br><br><br><br><br>








# L1和L2 programs功能

1. L2中SOL的增发与销毁  
   1. L2 SOL 增发,参考质押获得奖励,增加一个直接mint SOL的指令.并在L1向L2跨链情况下,产生的L2交易结构中增加mint字段,存放需要增发的SOL数量.
   2. L2 SOL 销毁(燃烧),在L2合约中增燃烧合约,在L2向L1跨链情况下,L2中SOL转入销毁合约,销毁合约提供燃烧方法,由外部定期调用.销毁实现为创建新的account,销毁合约中余额SOL转入新的account中,关闭account,将销毁转出sol地址设置为销毁account地址.
   
2. L1向L2跨链消息,由"跨链模块"协作,负责扫描L1 "Message" program中的跨链消息,Message 为每个跨链消息生成递增的nonce.跨链模块扫描时会协同本地nonce和向L2验证跨链消息是否成功.保持按L1 Message中nonce 递增原则,确保不漏扫.  
   
3. L2向L1跨链验证业务流  
   1. L2 Message Program中 实现SMT树,将L2向L1的 (跨链消息->bool) 结构存储向SMT树中.
   2. 用户调用 L2 Bridge进行跨链操作.
   3. L2 Bridge 将跨链信息发送至L2 Message,并将 SOL 发送至 Burn合约,待定期销毁.
   4. L2 Message 将跨链消息进行封装并存入SMT树中.
   5. Submit 模块会定期将L2 Message中SMT树中的Root,存入 L1 Root Program中.
   6. Submit 模块(或者 桥模块) 在向L1 Root 提交时,需要将完整的SMT 存储下来.
   7. 等待挑战周期.
   8. 用户调用Submit获取需要的Proof,将证明提交至 L1 Message中.
   9. 验证通过后,可进行提币操作.
    
4. L1 Programs 划分  
   1. Bridge  
      1. 存入SOL,分配nonce,调用 L1 Message.
   2. Message  
      1. 中继L1向L2消息,封装触发"跨链事件",和桥模块扫描配合.
      2. 验证L2向L1消息,通过index向L1 Root中查询L2 Message中SMT树的根,将用户提交的proof和跨链消息进行验证.
      3. 可将验证通过的消息进行执行(提币操作).
   3. Root
      1. 维护L2 Message中SMT状态根,以index递增的形式,存储对应的根.
   
5. L2 Programs 划分  
   1. Bridge
      1. 接受L2 Message 的调用,进行sol分发.
      2. 取出SOL,将消息进行封装发送至L2 Message.  
   2. Message
      1. 中继L2向L1消息,封装并存入SMT中.
      2. 接受桥模块生成的L1向L2的跨链交易.
   3. Burn
      1. 可接受SOL转账
      2. 销毁SOL,创建新的account,将SOL所有余额转入,关闭新的account.
   
6. 问题
   1. 链下谁来存储L2 Message合约中的全量SMT树.Submit or 桥模块
   2. 桥模块扫块实现.
      1. programSubscribe,实时订阅
         1. 优点:消息可快速触发跨链后续业务.
         2. 缺点:
            1. 无法获取订阅前的跨链消息
            2. 产生跨链消息不一致时,回归正常比较复杂.
      2. getConfirmedSignaturesForAddress2或getConfirmedTransaction
         1. 优点:随时能获得完整的跨链消息
         2. 缺点:效率不高,业务过程中存在大量解析和解码
      3. 将L1 Message中nonce和 nonce对应的跨链信息存下来.
         1. 优点:随时能获得完整的跨链消息并且方便查询历史.
         2. 缺点:费钱.

# 问题解决
1. L1到L2, 跨链消息通过nonce->PDA->跨链消息
2. Sync, 增加扫描L2 slot,搜查到L2到L1的跨链消息,tx -> slot高度,暂存入内存中.
3. 扫描L1 Root 合约,扫描到的slot高度内的交易,重构solt高度以内的交易所产生的SMT树.将结果和L1 Root合约中的Root对比.
4. 重构后SMT后,将这之间的跨链交易的Proof进行存储,  L2 tx -> (proof -> root)
   


# 模块规划

## 链上
1. L1 Programs 划分  
   1. Bridge  
      1. 存入SOL,分配nonce,调用 L1 Message.
   2. Message  
      1. 中继L1向L2消息,封装触发"跨链事件",和桥模块扫描配合.
      2. 验证L2向L1消息,通过index向L1 Root中查询L2 Message中SMT树的根,将用户提交的proof和跨链消息进行验证.
      3. 可将验证通过的消息进行执行(提币操作).
   3. Root
      1. 维护L2 Message中SMT状态根,以index递增的形式,存储对应的根.
   4. pool(未确定)
      1. 存储SOL,只有特定账户可进行取出.
   
2. L2 Programs 划分  
   1. Bridge
      1. 接受L2 Message 的调用,进行sol分发.
      2. 取出SOL,将消息进行封装发送至L2 Message.  
   2. Message
      1. 中继L2向L1消息,封装并存入SMT中.
      2. 接受桥模块生成的L1向L2的跨链交易.
   3. Burn
      1. 可接受SOL转账
      2. 销毁SOL,创建新的account,将SOL所有余额转入,关闭新的account.
   4. pool(未确定)
      1. 存储SOL,只有特定账户可进行取出.
## 链下
1. sync(同步模块)
   1. 扫描L1 Message合约.
   2. 根据L1 Message中跨链消息,nonce -> PDA -> 跨链消息,生成到L2 Message的跨链消息.
   3. 本地维护nonce,确保L1到L2跨链消息的双边一致.
   4. 
   5. 扫描L2 区块.
   6. 根据扫描L2 区块,将L2到L1的跨链消息暂存(user->L2 Bridge).
   7. 
   8. 本地维护SMT树.
   9.  扫描L1 Root合约.
   10. 结合Root合约,solt 高度,将L2 到L1的跨链交易同步到本地SMT树中,对比L1 Root合约中的根,一致则生成本次更新SMT交易的所有Proof(长期永久存储).
2. Submit(提交模块)
   1. 将L2世界状态根,solt高度,L2 Message中SMT根,存储至 L1 Root中.# sol-root
# sol-root
