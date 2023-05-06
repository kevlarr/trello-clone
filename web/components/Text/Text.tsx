import { ReactNode, ComponentPropsWithoutRef } from 'react'
import css from './Text.module.css'

// Pretty sure that `p` and `span` both have the same standard props,
// but intersect just to be safe?
// (See: https://react.dev/reference/react-dom/components/common)
type BaseProps = ComponentPropsWithoutRef<'p'> & ComponentPropsWithoutRef<'span'>

interface Props extends BaseProps {
  children?: ReactNode,
  inline?: boolean,
  strong?: boolean,
}

export default function Text({
  children,
  inline = false,
  strong = false,
  ...rest
} : Props) {
  const Tag = inline ? 'span' : 'p'
  let inner = children

  if (strong) {
    inner = <strong>{inner}</strong>
  }

  return (
    <Tag className={css.text} {...rest} >
      {inner}
    </Tag>
  )
}
