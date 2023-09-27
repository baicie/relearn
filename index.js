const arr = [
  { name: '66', sex: '男' },
  { name: 'hh', sex: '女' },
  { name: '99', sex: '男' },
  { name: '大哥', sex: '男' },
  { name: '巨型萝莉', sex: '女' }
]

function main(data, key) {
  const res = {}
  for (const item of data) {
    const inneKey = item[key]

    if (!res[inneKey]) {
      res[inneKey] = []
    }
    res[inneKey].push(item)
  }
  return res
}

const res = main(arr, 'sex')

console.log(res);