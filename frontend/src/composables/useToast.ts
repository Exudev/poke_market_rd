import { ref } from 'vue'

export type ToastType = 'success' | 'error' | 'info' | 'warning'

export interface Toast {
  id: number
  message: string
  type: ToastType
}

const toasts = ref<Toast[]>([])
let nextId = 0

export function useToast() {
  const addToast = (message: string, type: ToastType = 'info') => {
    const id = nextId++
    toasts.value.push({ id, message, type })
    setTimeout(() => {
      removeToast(id)
    }, 3000)
  }

  const removeToast = (id: number) => {
    const index = toasts.value.findIndex(t => t.id === id)
    if (index > -1) {
      toasts.value.splice(index, 1)
    }
  }

  const success = (msg: string) => addToast(msg, 'success')
  const error = (msg: string) => addToast(msg, 'error')
  const info = (msg: string) => addToast(msg, 'info')
  const warning = (msg: string) => addToast(msg, 'warning')

  return {
    toasts,
    success,
    error,
    info,
    warning,
    removeToast
  }
}
