import React from 'react'
import { Loader2 } from "lucide-react"

interface LoadingProps {
  message?: string
}

export const Loading: React.FC<LoadingProps> = ({
  message = "Loading dashboard..."
}) => {
  return (
    <div className="min-h-screen bg-gradient-to-br from-[#667eea] to-[#764ba2] flex items-center justify-center font-sans">
      <div className="text-center text-white p-8 bg-white/10 rounded-2xl">
        <Loader2 className="h-12 w-12 animate-spin mx-auto mb-4" />
        <div className="text-lg">{message}</div>
      </div>
    </div>
  )
}
