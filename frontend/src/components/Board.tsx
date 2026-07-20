import Square from './Square.tsx'

export default function Board() {
  return (
    <div className="grid grid-cols-8 grid-rows-8 w-[min(90vw,90vh)] h-[min(90vw,90vh)]">
      {Array(64)
        .fill(0)
        .map((_, i) => (
          <Square key={i} />
        ))}
    </div>
  )
}