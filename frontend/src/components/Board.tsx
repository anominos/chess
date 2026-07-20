import Square from './Square.tsx'

export default function Board() {
  return (
    <div className="w-[min(90vw,90vh)] h-[min(90vw,90vh)] rounded-2xl border border-slate-300 bg-white p-2 shadow-2xl shadow-black/20 dark:border-slate-700 dark:bg-slate-900 dark:shadow-black/40">
      <div className="grid h-full w-full grid-cols-8 grid-rows-8 overflow-hidden rounded-xl">
        {Array(64)
          .fill(0)
          .map((_, i) => (
            <Square key={i} index={i} />
          ))}
      </div>
    </div>
  )
}