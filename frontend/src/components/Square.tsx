type SquareProps = {
  index: number
}

export default function Square({ index }: SquareProps) {
  const isDark = (Math.floor(index / 8) + index) % 2 === 1

  return (
    <div
      className={`flex h-full w-full items-center justify-center border border-slate-300 dark:border-slate-700 ${
        isDark
          ? 'bg-slate-700 text-slate-100 dark:bg-slate-700 dark:text-slate-100'
          : 'bg-slate-100 text-slate-900 dark:bg-slate-800 dark:text-slate-100'
      }`}
    >
      <span className="text-xs font-medium opacity-60">{index + 1}</span>
    </div>
  )
}