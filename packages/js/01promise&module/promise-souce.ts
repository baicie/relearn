export class PromiseSource {
  status: 'pending' | 'fullfilled' | 'rejected' = 'pending'
  resolvedCallback = []
  rejectedCallback = []

  constructor(handlerFn) {
    this.status = 'pending'
    // 当调用resolve时调用实例中的trigger方法执行内置逻辑（触发then中方法）
    handlerFn(this.triggerResolvedCallback.bind(this), this.triggerRejectedCallback.bind(this))
  }

  triggerResolvedCallback(value: any) {
    // 获取resolve传递的值并触发then中捕获的依赖
  }

  triggerRejectedCallback(error: any) {
    //
  }

  then(onFulfilled, onRejected) {
    if (this.status !== 'pending')
      return
    // 捕获依赖
    if (typeof onFulfilled !== 'function')
      return
  }
}

const promise1 = new PromiseSource((resolve, reject) => {
  //
})
