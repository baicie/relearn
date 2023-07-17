export class PromiseSource {
  status: 'pending' | 'fullfilled' | 'rejected' = 'pending'
  value: any
  resolvedCallback = []
  rejectedCallback = []

  constructor(handleFunc) {
    this.status = 'pending'
    this.value = undefined
    // 当调用resolve时调用实例中的trigger方法执行内置逻辑（触发then中方法）
    handleFunc(this.triggerResolve.bind(this), this.triggerReject.bind(this))
  }

  triggerResolve(value: any) {
    // 获取resolve传递的值并触发then中捕获的依赖
    setTimeout(() => {
      if (this.status !== 'pending')
        return

      if (value instanceof PromiseSource) {
        value.then(
          (value) => {},
          (err) => {},
        )
      }
      else {
        this.status = 'fullfilled'
        this.value = value
        this.triggerFulfilled(value)
      }
    })
  }

  triggerFulfilled(val) {
    this.resolvedCallback.forEach(callback => callback(val))
    this.rejectedCallback = []
  }

  triggerReject(error: any) {
    //
  }

  then(onFulfilled, onRejected) {
    const { value, status } = this
    return new PromiseSource((onNextFulfilled, onNextRejected) => {
      function onFinalFulfilled(val) {
        if (typeof onFulfilled !== 'function') {
          onNextFulfilled(val)
        }
        else {
          const res = onFulfilled(val)
          if (res instanceof PromiseSource)
            res.then(onNextFulfilled, onNextRejected)

          else
            onNextFulfilled(res)
        }
      }

      function onFinalRejected(error) {
        if (typeof onRejected !== 'function') {
          onNextRejected(error)
        }
        else {
          let res = null
          try {
            res = onRejected(error)
          }
          catch (error) {
            onNextRejected(error)
          }

          if (res instanceof PromiseSource)
            res.then(onNextRejected, onNextRejected)

          else
            onFulfilled(res)
        }
      }

      switch (status) {
        case 'pending':{
          this.resolvedCallback.push(onFinalFulfilled)
          this.rejectedCallback.push(onFinalRejected)
          break
        }
        case 'fullfilled':{
          onFinalFulfilled(value)
          break
        }
        case 'rejected':{
          onFinalRejected(value)
          break
        }
      }
    })
  }

  caches(onRejected) {
    return this.then(null, onRejected)
  }

  static resolve(value) {
    if (value instanceof PromiseSource)
      return value
    return new PromiseSource(resolve => resolve(value))
  }

  static all(list) {
    return new PromiseSource((resolve, reject) => {
      let count = 0
      const values = []

      for (const [i, PromiseInstance] of Object.entries(list)) {
        PromiseSource.resolve(PromiseInstance)
          .then(
            (res) => {
              values[i] = res
              count++
              if (count === list.length)
                resolve(values)
            },
            (err) => {
              reject(err)
            },
          )
      }
    })
  }

  static race(list) {
    return new PromiseSource((resolve, reject) => {
      list.forEach((item) => {
        PromiseSource.resolve(item)
          .then(
            (res) => {
              resolve(res)
            },
            (err) => {
              reject(err)
            },
          )
      })
    })
  }
}

const promise1 = new PromiseSource((resolve, reject) => {
  resolve(1)
})

promise1.then((res) => {
  console.log('res', res)
})
