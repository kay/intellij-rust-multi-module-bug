# intellij-rust-multi-module-bug
Repository that replicates an issue with the intellij-rust plugin for multi-module builds, as with multi-language projects

# Environment
**IntelliJ Rust plugin version:** 0.3.139.3615-203

**Rust toolchain version:** 1.48.0-x86_64-unknown-linux-gnu

**IDE name and version:** IntelliJ IDEA 2020.3.1 Ultimate Edition (IU-203.6682.168)

**Operating system:** Linux

# Problem description

Fails to attach cargo project in a multi-module project. Error in log appears as:
```
2021-01-20 12:04:44,772 [2419557]   INFO - oject.model.impl.CargoSyncTask - CargoSyncTask started 
2021-01-20 12:04:44,774 [2419559]   INFO - rust.openapiext.CommandLineExt - Executing `/home/doug/.cargo/bin/rustc --print sysroot` 
2021-01-20 12:04:44,880 [2419665]   INFO - rust.openapiext.CommandLineExt - Executing `/home/doug/.cargo/bin/rustc --version --verbose` 
2021-01-20 12:04:45,279 [2420064]   INFO - rust.openapiext.CommandLineExt - Executing `/home/doug/.cargo/bin/rustc --print cfg` 
2021-01-20 12:04:45,380 [2420165]   INFO - rust.openapiext.CommandLineExt - Executing `/home/doug/.cargo/bin/rustup component list` 
2021-01-20 12:04:45,509 [2420294]   INFO - rust.openapiext.CommandLineExt - Executing `/home/doug/.cargo/bin/rustc --print sysroot` 
2021-01-20 12:04:45,611 [2420396]   INFO - t.cargo.toolchain.tools.Rustup - stdlib path: /home/doug/.rustup/toolchains/1.48.0-x86_64-unknown-linux-gnu/lib/rustlib/src/rust 
2021-01-20 12:04:46,249 [2421034]  ERROR -    #org.rust.stdext.AsyncValue - java.lang.RuntimeException: java.lang.IllegalStateException: Source folder file:///home/doug/src/personal/intellij-rust-multi-module-bug/b/src must be under content entry file:///home/doug/src/personal/intellij-rust-multi-module-bug/a 
java.util.concurrent.CompletionException: java.lang.RuntimeException: java.lang.IllegalStateException: Source folder file:///home/doug/src/personal/intellij-rust-multi-module-bug/b/src must be under content entry file:///home/doug/src/personal/intellij-rust-multi-module-bug/a
        at java.base/java.util.concurrent.CompletableFuture.encodeThrowable(CompletableFuture.java:314)
        at java.base/java.util.concurrent.CompletableFuture.completeThrowable(CompletableFuture.java:319)
        at java.base/java.util.concurrent.CompletableFuture$UniApply.tryFire(CompletableFuture.java:645)
        at java.base/java.util.concurrent.CompletableFuture.postComplete(CompletableFuture.java:506)
        at java.base/java.util.concurrent.CompletableFuture.complete(CompletableFuture.java:2073)
        at org.rust.cargo.project.model.impl.CargoSyncTask.run(CargoSyncTask.kt:87)
        at com.intellij.openapi.progress.impl.CoreProgressManager$TaskRunnable.run(CoreProgressManager.java:962)
        at com.intellij.openapi.progress.impl.CoreProgressManager.lambda$runProcessWithProgressAsync$5(CoreProgressManager.java:472)
        at com.intellij.openapi.progress.impl.ProgressRunner.lambda$submit$3(ProgressRunner.java:235)
        at com.intellij.openapi.progress.impl.CoreProgressManager.lambda$runProcess$2(CoreProgressManager.java:178)
        at com.intellij.openapi.progress.impl.CoreProgressManager.registerIndicatorAndRun(CoreProgressManager.java:658)
        at com.intellij.openapi.progress.impl.CoreProgressManager.executeProcessUnderProgress(CoreProgressManager.java:610)
        at com.intellij.openapi.progress.impl.ProgressManagerImpl.executeProcessUnderProgress(ProgressManagerImpl.java:65)
        at com.intellij.openapi.progress.impl.CoreProgressManager.runProcess(CoreProgressManager.java:165)
        at com.intellij.openapi.progress.impl.ProgressRunner.lambda$submit$4(ProgressRunner.java:235)
        at java.base/java.util.concurrent.CompletableFuture$AsyncSupply.run(CompletableFuture.java:1700)
        at java.base/java.util.concurrent.ThreadPoolExecutor.runWorker(ThreadPoolExecutor.java:1128)
        at java.base/java.util.concurrent.ThreadPoolExecutor$Worker.run(ThreadPoolExecutor.java:628)
        at java.base/java.util.concurrent.Executors$PrivilegedThreadFactory$1$1.run(Executors.java:668)
        at java.base/java.util.concurrent.Executors$PrivilegedThreadFactory$1$1.run(Executors.java:665)
        at java.base/java.security.AccessController.doPrivileged(Native Method)
        at java.base/java.util.concurrent.Executors$PrivilegedThreadFactory$1.run(Executors.java:665)
        at java.base/java.lang.Thread.run(Thread.java:834)
Caused by: java.lang.RuntimeException: java.lang.IllegalStateException: Source folder file:///home/doug/src/personal/intellij-rust-multi-module-bug/b/src must be under content entry file:///home/doug/src/personal/intellij-rust-multi-module-bug/a
        at com.intellij.openapi.application.impl.LaterInvocator.invokeAndWait(LaterInvocator.java:149)
        at com.intellij.openapi.application.impl.ApplicationImpl.invokeAndWait(ApplicationImpl.java:476)
        at com.intellij.openapi.application.ActionsKt.invokeAndWaitIfNeeded(actions.kt:34)
        at com.intellij.openapi.application.ActionsKt.invokeAndWaitIfNeeded$default(actions.kt:27)
        at org.rust.cargo.project.model.impl.CargoProjectImplKt.setupProjectRoots(CargoProjectImpl.kt:553)
        at org.rust.cargo.project.model.impl.CargoProjectImplKt.access$setupProjectRoots(CargoProjectImpl.kt:1)
        at org.rust.cargo.project.model.impl.CargoProjectImplKt$doRefresh$1.apply(CargoProjectImpl.kt:538)
        at org.rust.cargo.project.model.impl.CargoProjectImplKt$doRefresh$1.apply(CargoProjectImpl.kt)
        at java.base/java.util.concurrent.CompletableFuture$UniApply.tryFire(CompletableFuture.java:642)
        ... 20 more
Caused by: java.lang.IllegalStateException: Source folder file:///home/doug/src/personal/intellij-rust-multi-module-bug/b/src must be under content entry file:///home/doug/src/personal/intellij-rust-multi-module-bug/a
        at com.intellij.workspaceModel.ide.impl.legacyBridge.module.roots.ModifiableContentEntryBridge.addSourceFolder(ModifiableContentEntryBridge.kt:46)
        at com.intellij.workspaceModel.ide.impl.legacyBridge.module.roots.ModifiableContentEntryBridge.addSourceFolder(ModifiableContentEntryBridge.kt:207)
        at com.intellij.workspaceModel.ide.impl.legacyBridge.module.roots.ModifiableContentEntryBridge.addSourceFolder(ModifiableContentEntryBridge.kt:201)
        at com.intellij.workspaceModel.ide.impl.legacyBridge.module.roots.ModifiableContentEntryBridge.addSourceFolder(ModifiableContentEntryBridge.kt:198)
        at org.rust.cargo.project.model.CargoProjectServiceKt.setup(CargoProjectService.kt:166)
        at org.rust.cargo.project.model.impl.CargoProjectImplKt$setupProjectRoots$1$1$1$3$1.invoke(CargoProjectImpl.kt:574)
        at org.rust.cargo.project.model.impl.CargoProjectImplKt$setupProjectRoots$1$1$1$3$1.invoke(CargoProjectImpl.kt)
        at org.rust.cargo.project.model.impl.CargoProjectImplKt$setupContentRoots$1.consume(CargoProjectImpl.kt:610)
        at org.rust.cargo.project.model.impl.CargoProjectImplKt$setupContentRoots$1.consume(CargoProjectImpl.kt)
        at com.intellij.openapi.roots.ModuleRootModificationUtil.lambda$updateModel$8(ModuleRootModificationUtil.java:126)
        at com.intellij.openapi.roots.ModuleRootModificationUtil.modifyModel(ModuleRootModificationUtil.java:134)
        at com.intellij.openapi.roots.ModuleRootModificationUtil.updateModel(ModuleRootModificationUtil.java:125)
        at org.rust.cargo.project.model.impl.CargoProjectImplKt.setupContentRoots(CargoProjectImpl.kt:609)
        at org.rust.cargo.project.model.impl.CargoProjectImplKt.access$setupContentRoots(CargoProjectImpl.kt:1)
        at org.rust.cargo.project.model.impl.CargoProjectImplKt$setupProjectRoots$1$$special$$inlined$runWriteAction$1$lambda$1$1.invoke(CargoProjectImpl.kt:574)
        at org.rust.cargo.project.model.impl.CargoProjectImplKt$setupProjectRoots$1$$special$$inlined$runWriteAction$1$lambda$1$1.invoke(CargoProjectImpl.kt:584)
        at org.rust.cargo.project.model.impl.CargoProjectImplKt$setupProjectRoots$1$$special$$inlined$runWriteAction$1$lambda$1.run(CargoProjectImpl.kt:595)
        at com.intellij.openapi.roots.impl.ProjectRootManagerImpl.mergeRootsChangesDuring(ProjectRootManagerImpl.java:402)
        at org.rust.cargo.project.model.impl.CargoProjectImplKt$setupProjectRoots$1$$special$$inlined$runWriteAction$1.compute(actions.kt:62)
        at com.intellij.openapi.application.impl.ApplicationImpl.lambda$runWriteAction$16(ApplicationImpl.java:1009)
        at com.intellij.openapi.application.impl.ApplicationImpl.runWriteActionWithClass(ApplicationImpl.java:988)
        at com.intellij.openapi.application.impl.ApplicationImpl.runWriteAction(ApplicationImpl.java:1009)
        at org.rust.cargo.project.model.impl.CargoProjectImplKt$setupProjectRoots$1.invoke(CargoProjectImpl.kt:614)
        at org.rust.cargo.project.model.impl.CargoProjectImplKt$setupProjectRoots$1.invoke(CargoProjectImpl.kt)
        at com.intellij.openapi.application.ActionsKt$invokeAndWaitIfNeeded$$inlined$computeDelegated$lambda$1.run(actions.kt:34)
        at com.intellij.openapi.application.TransactionGuardImpl.runWithWritingAllowed(TransactionGuardImpl.java:216)
        at com.intellij.openapi.application.TransactionGuardImpl.access$200(TransactionGuardImpl.java:24)
        at com.intellij.openapi.application.TransactionGuardImpl$2.run(TransactionGuardImpl.java:199)
        at com.intellij.openapi.application.impl.ApplicationImpl.runIntendedWriteActionOnCurrentThread(ApplicationImpl.java:822)
        at com.intellij.openapi.application.impl.ApplicationImpl.lambda$invokeAndWait$8(ApplicationImpl.java:476)
        at com.intellij.openapi.application.impl.LaterInvocator$1.run(LaterInvocator.java:126)
        at com.intellij.openapi.application.impl.FlushQueue.doRun(FlushQueue.java:85)
        at com.intellij.openapi.application.impl.FlushQueue.runNextEvent(FlushQueue.java:134)
        at com.intellij.openapi.application.impl.FlushQueue.flushNow(FlushQueue.java:47)
        at com.intellij.openapi.application.impl.FlushQueue$FlushNow.run(FlushQueue.java:190)
        at java.desktop/java.awt.event.InvocationEvent.dispatch(InvocationEvent.java:313)
        at java.desktop/java.awt.EventQueue.dispatchEventImpl(EventQueue.java:776)
        at java.desktop/java.awt.EventQueue$4.run(EventQueue.java:727)
        at java.desktop/java.awt.EventQueue$4.run(EventQueue.java:721)
        at java.base/java.security.AccessController.doPrivileged(Native Method)
        at java.base/java.security.ProtectionDomain$JavaSecurityAccessImpl.doIntersectionPrivilege(ProtectionDomain.java:85)
        at java.desktop/java.awt.EventQueue.dispatchEvent(EventQueue.java:746)
        at com.intellij.ide.IdeEventQueue.defaultDispatchEvent(IdeEventQueue.java:976)
        at com.intellij.ide.IdeEventQueue._dispatchEvent(IdeEventQueue.java:843)
        at com.intellij.ide.IdeEventQueue.lambda$dispatchEvent$8(IdeEventQueue.java:454)
        at com.intellij.openapi.progress.impl.CoreProgressManager.computePrioritized(CoreProgressManager.java:773)
        at com.intellij.ide.IdeEventQueue.lambda$dispatchEvent$9(IdeEventQueue.java:453)
        at com.intellij.openapi.application.impl.ApplicationImpl.runIntendedWriteActionOnCurrentThread(ApplicationImpl.java:822)
        at com.intellij.ide.IdeEventQueue.dispatchEvent(IdeEventQueue.java:501)
        at java.desktop/java.awt.EventDispatchThread.pumpOneEventForFilters(EventDispatchThread.java:203)
        at java.desktop/java.awt.EventDispatchThread.pumpEventsForFilter(EventDispatchThread.java:124)
        at java.desktop/java.awt.EventDispatchThread.pumpEventsForHierarchy(EventDispatchThread.java:113)
        at java.desktop/java.awt.EventDispatchThread.pumpEvents(EventDispatchThread.java:109)
        at java.desktop/java.awt.EventDispatchThread.pumpEvents(EventDispatchThread.java:101)
        at java.desktop/java.awt.EventDispatchThread.run(EventDispatchThread.java:90)
2021-01-20 12:04:46,250 [2421035]  ERROR -    #org.rust.stdext.AsyncValue - IntelliJ IDEA 2020.3.1  Build #IU-203.6682.168 
2021-01-20 12:04:46,250 [2421035]  ERROR -    #org.rust.stdext.AsyncValue - JDK: 11.0.9.1; VM: OpenJDK 64-Bit Server VM; Vendor: JetBrains s.r.o. 
2021-01-20 12:04:46,250 [2421035]  ERROR -    #org.rust.stdext.AsyncValue - OS: Linux 
2021-01-20 12:04:46,251 [2421036]  ERROR -    #org.rust.stdext.AsyncValue - Plugin to blame: Rust version: 0.3.139.3615-203 
2021-01-20 12:04:46,251 [2421036]  ERROR -    #org.rust.stdext.AsyncValue - Last Action: Cargo.AttachCargoProject 
```

# Steps to reproduce

To replicate, open the project from IntelliJ. Then attach the cargo project either from the cargo tool window, or by
opening a `*.rs` or `*.toml` file and clicking `Attach` when prompted.

If you comment out the contents of `settings.gradle` and refresh gradle, then try again this works.

# Notes

This appears to be sensitive to the order each package is listed in `cargo  metadata --verbose --format-version 1 --all-features`.
As such, if you rename the `a` crate to `c`, the cargo sync will be successful.

