export default function Piece() {
  return (
    <div
      draggable
      className="flex h-20 w-20 items-center justify-center rounded-full bg-slate-500 text-slate-900 shadow-sm dark:bg-slate-500 dark:text-slate-100"
    >
      <span className="text-xl font-bold">P</span>
    </div>
  );
}
